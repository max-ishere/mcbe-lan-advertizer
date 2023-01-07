use anyhow::Context;
use clap::Parser;
use mcbe_lan_advertizer::{
    config::{
        args::{Args, Mode},
        server_properties::ServerProperties,
    },
    pong::Pong,
};
use std::{fs::read_to_string, net::UdpSocket, str::FromStr};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.mode {
        Mode::Relay { props } => {
            println!("Relay mode");
            let properties = ServerProperties::from_str(&read_to_string(props)?)?;
            relay(properties)?;
        }
        Mode::FromProps { props, .. } => {
            println!("server.properties mode");
            let properties = ServerProperties::from_str(&read_to_string(props)?)?;
            from_properties(properties)?;
        }
    }
    Ok(())
}

fn relay(server_properties: ServerProperties) -> anyhow::Result<()> {
    let clients = UdpSocket::bind(("0.0.0.0", 19132)).with_context(|| {
        "Could not open client-facing socket on port 19132. Is the Bedrock server (or other program) running on that port?"
    })?;
    let server = UdpSocket::bind(("0.0.0.0", 0)).with_context(|| {
        "Could not open server-facing socket. The socket address is random and assigned by the system, so look for details below."
    })?;

    println!("Ready to relay client connections on localhost:19132");
    println!(
        "Server relay socket open on localhost:{server_port}, do not send connections there!",
        server_port = server.local_addr()?.port()
    );

    let mut buf = [0u8; 1000];

    loop {
        let (len, client_addr) = clients
            .recv_from(&mut buf)
            .with_context(|| "Failed to read a client Ping packet.")?;

        // TODO: make sure this is actually Ping packet
        let ping = &mut buf[..len];

        server
            .send_to(ping, ("0.0.0.0", server_properties.port4))
            .with_context(|| "Failed to send client's Ping to the server.")?;

        let (len, server_addr) = server
            .recv_from(&mut buf)
            .with_context(|| "Failed to read server's Pong packet.")?;
        if server_addr.port() != server_properties.port4 {
            continue;
        }

        let pong = &buf[..len];
        clients
            .send_to(pong, client_addr)
            .with_context(|| "Failed to send server's Pong to the client")?;
    }
}

fn from_properties(server_properties: ServerProperties) -> anyhow::Result<()> {
    let clients = UdpSocket::bind(("0.0.0.0", 19132)).with_context(|| {
        "Could not open a UDP socket on localhost:19132. Is the Bedrock server (or other program) running on this port?"
    })?;

    println!("Ready to Pong using server.properties to client connections on localhost:19132");

    let mut buf = [0u8; 1000];
    let pong = Pong::from(server_properties).as_bytes();

    loop {
        let (len, client_addr) = clients
            .recv_from(&mut buf)
            .with_context(|| "Failed to read client's Ping packet")?;

        // TODO: make sure this is actually Ping packet
        let ping = &mut buf[..len];

        clients
            .send_to(&pong, client_addr)
            .with_context(|| "Failed to send a Pong to the client")?;
    }
}
