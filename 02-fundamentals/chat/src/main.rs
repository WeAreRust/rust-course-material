#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

mod client;
mod protocol;
mod server;

use clap::{App, Arg, ArgMatches, SubCommand};
use client::Client;

use protocol::{Datagram, PublishDatagram};
use server::Server;
use std::net::SocketAddrV4;
use std::time::Duration;

const DEFAULT_UDP_PORT: u16 = 31337;

fn main() {
    env_logger::init();

    let app = App::new("My Super Program")
        .version(crate_version!())
        .author("Benjamin Thompson <me@benjaminjt.com>")
        .about("UDP chat application for #rust-club")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Select a port to operate on")
                .takes_value(true)
                .validator(validate_u16_arg),
        )
        .subcommand(SubCommand::with_name("server").about("Runs the server"))
        .subcommand(
            SubCommand::with_name("client")
                .about("Runs the client")
                .arg(
                    Arg::with_name("server_address")
                        .short("a")
                        .long("address")
                        .value_name("SERVER_IP")
                        .help("IPv4 address of the server")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("channel")
                        .short("c")
                        .long("channel")
                        .value_name("CHANNEL")
                        .help("Channel(s) to subscribe to")
                        .multiple(true)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .value_name("MESSAGE")
                        .help("Message to send on connect, or each each interval")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("interval")
                        .short("i")
                        .long("interval")
                        .value_name("MESSAGE_INTERVAL")
                        .help("Interval to send messages")
                        .takes_value(true)
                        .validator(validate_usize_arg),
                ),
        )
        .get_matches();


    let port = match app.value_of("port") {
        Some(s) => s.parse().unwrap(),
        None => DEFAULT_UDP_PORT,
    };

    match app.subcommand() {
        ("server", _) => run_server(port),
        ("client", Some(client_app)) => run_client(port, client_app),
        _ => panic!("No subcommand provided! To see usage, use the 'help' subcommand."),
    }
}

fn validate_usize_arg(s: String) -> Result<(), String> {
    let result: Result<usize, std::num::ParseIntError> = s.parse();
    result
        .map(|_| ())
        .map_err(|_| format!("Could not parse '{}' as usize", s))
}

fn validate_u16_arg(s: String) -> Result<(), String> {
    let result: Result<u16, std::num::ParseIntError> = s.parse();
    result
        .map(|_| ())
        .map_err(|_| format!("Could not parse '{}' as u16", s))
}

pub fn run_client(port: u16, app: &ArgMatches) {
    let address = app.value_of("server_address").unwrap();
    let channels_arg = app.values_of("channel");
    let message = app
        .value_of("message")
        .map(|s| Datagram::Publish(PublishDatagram::parse(s).unwrap()));
    let interval: Option<Duration> = app
        .value_of("interval")
        .map(|s| Duration::from_secs(s.parse().unwrap()));
    let client = Client::new(port, SocketAddrV4::new(address.parse().unwrap(), port)).unwrap();

    if let Some(channels) = channels_arg {
        channels.for_each(|channel| client.send(&Datagram::subscribe(channel)).unwrap());
    }

    loop {
        // Send a message
        if let Some(datagram) = &message {
            client.send(datagram).unwrap();
        }

        // Listen for the interval (this should block forever if it is None)
        if let Some(datagram) = client.listen(interval) {
            println!("{}", datagram.serialize());
        } else {
            debug!("No messages recieved...");
        }
    }
}

pub fn run_server(port: u16) -> ! {
    Server::new(port).unwrap().run()
}

