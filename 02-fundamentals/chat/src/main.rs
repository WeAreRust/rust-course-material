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

const DEFAULT_UDP_PORT: u16 = 31337;

fn main() {
    env_logger::init();

    let app = App::new("Rust Club Chat!")
        .version(crate_version!())
        .author("Benjamin Thompson <me@benjaminjt.com>")
        .about("UDP chat application for #rust-club")
        .subcommand(
            SubCommand::with_name("server")
                .about("Runs the server")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .value_name("PORT")
                        .help("Select a port to listen on")
                        .takes_value(true)
                        .validator(validate_u16_arg),
                ),
        )
        .subcommand(
            SubCommand::with_name("client")
                .about("Runs the client")
                .arg(
                    Arg::with_name("server_address")
                        .short("s")
                        .long("server")
                        .value_name("SERVER_ADDRESS")
                        .help("IPv4 address of the server (e.g. 127.0.0.1:1337)")
                        .validator(validate_ipv4_address)
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
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("message")
                        .short("m")
                        .long("message")
                        .value_name("MESSAGE")
                        .help("Message to send on connect")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match app.subcommand() {
        ("server", Some(server_app)) => run_server(server_app),
        ("client", Some(client_app)) => run_client(client_app),
        _ => panic!("No subcommand provided! To see usage, use the 'help' subcommand."),
    }
}

fn validate_u16_arg(s: String) -> Result<(), String> {
    let result: Result<u16, std::num::ParseIntError> = s.parse();
    result.map(|_| ()).map_err(|error| format!("{}", error))
}

fn validate_ipv4_address(s: String) -> Result<(), String> {
    let result: Result<SocketAddrV4, std::net::AddrParseError> = s.parse();
    result.map(|_| ()).map_err(|error| format!("{}", error))
}

pub fn run_client(app: &ArgMatches) {
    let server_address_arg = app.value_of("server_address").unwrap();
    let channels_arg = app.values_of("channel");
    let message_arg = app
        .value_of("message")
        .map(|s| Datagram::Publish(PublishDatagram::parse(s).unwrap()));

    let client = Client::new(0, server_address_arg.parse().unwrap()).unwrap();

    // Send a message
    if let Some(message) = &message_arg {
        client.send(message).unwrap();
    }

    if let Some(channels) = channels_arg {
        for channel in channels {
            client.send(&Datagram::subscribe(channel)).unwrap();
        }
    } else {
        // Nothing else to do if we're not subscribing
        return;
    }

    loop {
        if let Some(datagram) = client.listen(None) {
            println!("{}", datagram.serialize());
        } else {
            debug!("No messages recieved...");
        }
    }
}

pub fn run_server(server_app: &ArgMatches) -> ! {
    let port = match server_app.value_of("port") {
        Some(s) => s.parse().unwrap(),
        None => DEFAULT_UDP_PORT,
    };

    debug!("Running server on port: {}", port);
    Server::new(port).unwrap().run()
}

