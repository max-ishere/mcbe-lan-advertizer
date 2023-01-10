use anyhow::Context;
use clap::Parser;
use std::{fs::read_to_string, net::UdpSocket, str::FromStr, time::Duration};

use mcbe_lan_advertizer::{
    config::{
        args::{Args, Mode, Proto},
        server_properties::ServerProperties,
    },
    pong::Pong,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.mode {
        Mode::Relay { props } => {
            let properties =
                ServerProperties::from_str(&read_to_string(props.clone()).with_context(|| {
                    format!("Could not load server.properties at location: {props}",)
                })?)?;
            println!("Read server.properties at location: {props}");
            println!("Starting the relay. Make sure the server is running.");
            relay(properties)?;
        }
        Mode::FromProps { props, proto } => {
            let properties =
                ServerProperties::from_str(&read_to_string(props.clone()).with_context(|| {
                    format!("Could not load server.properties at location: {props}",)
                })?)?;
            println!("Read server.properties at location: {props}");
            println!("Starting the advertizer based on server.properties file. Player count will not be accurate.");
            match proto {
                Proto::Proto { numeric, string } => {
                    from_properties(properties, (numeric, string))?;
                }
            };
        }
    }
    Ok(())
}

fn relay(server_properties: ServerProperties) -> anyhow::Result<()> {
    let timeout = 10;

    let clients = create_socket(19132, timeout).with_context(|| {
        "Could not open client-facing socket on port 19132. Is the Bedrock server (or other program) running on that port?"
    }).and_then(|s| {
        println!(
            "Client-facing socket ready: {}",
            s
                .local_addr()
                .with_context(|| "Could not obtain client socket address")?
        );
        Ok(s)
    })?;
    clients
        .set_read_timeout(None)
        .with_context(|| "Could not reset read timeout")?;

    let server = create_socket(0, timeout).with_context(|| {
        "Could not open server-facing socket. The socket address is random and assigned by the system, so look for error details below."
    }).and_then(|s| {
        println!(
            "Server-facing socket ready: {}",
            s
                .local_addr()
                .with_context(|| "Could not obtain server socket address")?
        );
        Ok(s)
    })?;

    println!("Considering any network delay above {timeout}s as timed out, thus such connections will be dropped.");

    println!("Expecting server to be on port {}", server_properties.port4);
    println!("Setup complete, waiting for client connections.");

    let mut buf = [0u8; 0x2000];

    loop {
        let (len, client_addr) = match clients
            .recv_from(&mut buf)
            .with_context(|| "Failed to read a client Ping packet.")
        {
            Ok(ok) => ok,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        println!("{client_addr} pinged me.");

        // TODO: make sure this is actually Ping packet
        let ping = &mut buf[..len];

        println!("Waiting for server's responce.");
        if let Err(e) = server
            .send_to(ping, ("0.0.0.0", server_properties.port4))
            .with_context(|| "Failed to send client's Ping to the server.")
        {
            println!("{e}");
        }

        let len = match server
            .recv_from(&mut buf)
            .with_context(|| "Server Pong timed out, dropping this request.")
        {
            Ok((len, addr)) => {
                if addr.port() != server_properties.port4 {
                    continue;
                }
                len
            }

            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        println!("Got Pong from server, sending to the client");

        let pong = &buf[..len];
        clients
            .send_to(pong, client_addr)
            .with_context(|| "Failed to send server's Pong to the client")?;
        println!("Pong sent to client.");
    }
}

fn from_properties(
    server_properties: ServerProperties,
    proto: (u64, String),
) -> anyhow::Result<()> {
    let timeout = 10;
    let clients = create_socket(19132, timeout).with_context(|| {
        "Could not open a UDP socket on localhost:19132. Is the Bedrock server (or other program) running on this port?"
    })?;
    clients
        .set_read_timeout(None)
        .with_context(|| "Failed to reset client's read timeout.")?;

    println!("Client-facing socket ready on port 19132");
    println!(
        "Not interacting with a server in this mode. All information comes from server.properties"
    );
    println!("Considering any network delays above {timeout}s to be timed out, thus will drop such connections.");

    let mut buf = [0u8; 0x2000];
    let mut pong = Pong::from(server_properties);
    pong.protocol = proto;
    let pong = pong.as_bytes();

    println!("Setup complete, waiting for client connections.");

    loop {
        let (len, client_addr) = clients
            .recv_from(&mut buf)
            .with_context(|| "Failed to read client's Ping packet")?;
        println!("{client_addr} Pinged me.");

        // TODO: make sure this is actually Ping packet
        let ping = &mut buf[..len];

        clients
            .send_to(&pong, client_addr)
            .with_context(|| "Failed to send a Pong to the client")?;
        println!("Ponged {client_addr}");
    }
}

/// timeout: time to wait in seconds
fn create_socket(port: u16, timeout: u64) -> anyhow::Result<UdpSocket> {
    let socket = UdpSocket::bind(("0.0.0.0", port))
        .with_context(|| format!("Could not open UDP socket on port {port}"))?;
    socket.set_write_timeout(Some(Duration::from_secs(timeout))).with_context(||"Could not set write timeout. If message sending isn't time restricted the application may freeze.")?;
    socket.set_read_timeout(Some(Duration::from_secs(timeout))).with_context(||"Could not set read timeout. If the server does not respond quickly the application may freeze.")?;
    Ok(socket)
}
