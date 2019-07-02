use crate::protocol::{self, Datagram};
use std::io::Error;
use std::net::{UdpSocket, SocketAddrV4, Ipv4Addr};
use std::str::from_utf8;
use std::time::Duration;

pub struct Client {
    socket: UdpSocket,
    server_address: SocketAddrV4,
}

impl Client {
    pub fn new(port: u16, server_address: SocketAddrV4) -> Result<Self, Error> {
        let socket = UdpSocket::bind(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port))?;
        Ok(Client { socket, server_address })
    }

    pub fn send(&self, datagram: &Datagram) -> Result<(), Error> {
        self.socket.send_to(datagram.serialize().as_bytes(), self.server_address)?;
        Ok(())
    }

    fn parse_datagram(&self, buf: &[u8]) -> Option<Datagram> {
        let message = match from_utf8(&buf) {
            Ok(message) => message,
            Err(error) => {
                error!("Failed to parse datagram as UTF8: {}", error);
                return None;
            }
        };

        match Datagram::parse(message) {
            Ok(datagram) => Some(datagram),
            Err(protocol::Error::BadDatagram(message)) => {
                error!("Failed to parse datagram: {}", message);
                None
            }
        }
    }

    pub fn listen(&self, timeout: Option<Duration>) -> Option<Datagram> {
        self.socket
            .set_read_timeout(timeout)
            .unwrap_or_else(|error| {
                error!("Failed to set read timeout: {}", error);
            });

        let mut buf = [0; 1024];
        match self.socket.recv(&mut buf) {
            Ok(n) => self.parse_datagram(&buf[..n]),
            Err(error) => {
                let datagram = from_utf8(&buf).unwrap_or("[unable to parse UTF8]");
                error!("Failed to receive datagram: '{}'. {}", datagram, error);
                None
            }
        }
    }
}
