use crate::protocol::{Datagram, PublishDatagram, SubscribeDatagram, UnsubscribeDatagram};
use std::collections::{HashMap, HashSet};
use std::io::Error;
use std::iter::{once, FromIterator};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

use std::str;
use std::time::Duration;

pub struct Server {
    socket: UdpSocket,
    subscriptions: HashMap<String, HashSet<SocketAddr>>,
}

const WRITE_TIMEOUT: Option<Duration> = Some(Duration::from_secs(1));
const READ_TIMEOUT: Option<Duration> = None;

impl Server {
    pub fn new(port: u16) -> Result<Self, Error> {
        let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port))?;
        socket.set_read_timeout(READ_TIMEOUT)?;
        socket.set_write_timeout(WRITE_TIMEOUT)?;
        Ok(Server {
            socket,
            subscriptions: HashMap::new(),
        })
    }

    pub fn send(&self, publish_datagram: &PublishDatagram, address: &SocketAddr) {
        let datagram = Datagram::Publish(publish_datagram.copy());
        let result = self
            .socket
            .send_to(datagram.serialize().as_bytes(), address);
        if let Err(error) = result {
            error!("Error sending datagram: {}", error);
        }
    }

    fn handle_subscribe(&mut self, datagram: SubscribeDatagram, address: SocketAddr) {
        match self.subscriptions.get_mut(&datagram.channel) {
            Some(addresses) => {
                addresses.insert(address);
            }
            None => {
                let addresses = HashSet::from_iter(once(address));
                self.subscriptions.insert(datagram.channel, addresses);
            }
        };
    }

    fn handle_unsubscribe(&mut self, datagram: UnsubscribeDatagram, address: SocketAddr) {
        if let Some(addresses) = self.subscriptions.get_mut(&datagram.channel) {
            addresses.remove(&address);
        };
    }

    fn handle_publish(&mut self, datagram: PublishDatagram) {
        if let Some(addresses) = self.subscriptions.get(&datagram.channel) {
            for address in addresses.into_iter() {
                self.send(&datagram, &address);
            }
        };
    }

    fn handle_datagram(&mut self, datagram: Datagram, address: SocketAddr) {
        debug!("Handling: {}", datagram.serialize());

        match datagram {
            Datagram::Subscribe(d) => self.handle_subscribe(d, address),
            Datagram::Unsubscribe(d) => self.handle_unsubscribe(d, address),
            Datagram::Publish(d) => self.handle_publish(d),
            Datagram::Error(e) => {
                debug!("Recieved Datagram::Error: {}", e);
            }
        };
    }

    fn handle_datagram_string(&mut self, string: &str, address: SocketAddr) {
        match Datagram::parse(string) {
            Ok(datagram) => self.handle_datagram(datagram, address),
            Err(error) => {
                error!("Error parsing datagram: {:?}", error);
            }
        }
    }

    fn handle_datagram_buffer(&mut self, buf: &mut [u8], address: SocketAddr) {
        match str::from_utf8(buf) {
            Ok(string) => self.handle_datagram_string(string, address),
            Err(error) => {
                error!("Error parsing buffer to UTF8: {}", error);
            }
        }
    }

    fn handle_next(&mut self) {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((n, address)) => self.handle_datagram_buffer(&mut buf[..n], address),
            Err(error) => {
                error!("Error recieving next message: {}", error);
            }
        }
    }

    pub fn run(mut self) -> ! {
        loop {
            self.handle_next();
        }
    }
}

#[cfg(test)]
mod protocol_tests {
    use super::*;
    use crate::Client;
    use std::thread;

    fn loopback(port: u16) -> SocketAddrV4 {
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port)
    }

    fn test_server() -> (Server, SocketAddr) {
        let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)).unwrap();

        socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();

        let address = socket.local_addr().unwrap();
        let server = Server {
            socket,
            subscriptions: HashMap::new(),
        };

        (server, address)
    }

    fn test_client(server_port: u16) -> Client {
        Client::new(0, loopback(server_port)).unwrap()
    }

    #[test]
    fn basic_server() {
        let (mut server, server_address) = test_server();
        let server_port = server_address.port();

        let server_thread = thread::spawn(move || {
            // handle a max of 10 events
            for _ in 0..10 {
                server.handle_next();
            }
        });

        let client_thread_1 = thread::spawn(move || {
            let client_1 = test_client(server_port);
            client_1.send(&Datagram::subscribe("testing123")).unwrap();
            client_1.send(&Datagram::subscribe("nope")).unwrap();
            client_1.send(&Datagram::unsubscribe("nope")).unwrap();

            assert_eq!(
                client_1.listen(Some(Duration::from_millis(200))),
                Some(Datagram::publish("testing123", "sender", "hi clients!"))
            );

            assert_eq!(client_1.listen(Some(Duration::from_millis(200))), None);
        });

        let client_thread_2 = thread::spawn(move || {
            let client_2 = test_client(server_port);
            client_2.send(&Datagram::subscribe("testing123")).unwrap();
            client_2.send(&Datagram::subscribe("client2")).unwrap();

            assert_eq!(
                client_2.listen(Some(Duration::from_millis(200))),
                Some(Datagram::publish("testing123", "sender", "hi clients!"))
            );

            assert_eq!(
                client_2.listen(Some(Duration::from_millis(200))),
                Some(Datagram::publish("client2", "sender", "hi client 2!"))
            );

            assert_eq!(client_2.listen(Some(Duration::from_millis(200))), None);
        });

        // zzzz
        thread::sleep(Duration::from_millis(100));

        let sender = test_client(server_port);

        sender
            .send(&Datagram::publish("testing123", "sender", "hi clients!"))
            .unwrap();

        sender
            .send(&Datagram::publish("client2", "sender", "hi client 2!"))
            .unwrap();

        sender
            .send(&Datagram::publish("nope", "sender", "bad!"))
            .unwrap();


        server_thread.join().unwrap();
        client_thread_1.join().unwrap();
        client_thread_2.join().unwrap();
    }
}