//! [`Pong`] Represents a UDP raknet Pong packet as a struct
//!
//! You can create it from [`ServerProperties`] using the [`From`] trait

use rand::Rng;
use std::time::Instant;

use crate::config::server_properties::{Gamemode, ServerProperties};

const PONG: u8 = 0x1c;
type Uuid = u64;
const MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];
type StringLength = u16;

/// Represents a Pong packet
///
/// You can generate a Pong packet from [`ServerProperties`] or use the [`Default::default()`]
///
/// Use [`as_bytes()`](`Pong::as_bytes()`) to render a RakNet Pong packet
pub struct Pong {
    /// Used internally as part of the Raknet protocol
    init_time: Instant,
    /// Seems like this is just a random number. Official servers seem to have
    /// cool GUIDs like `8` but it does not seem to matter too much for a LAN
    /// server.
    guid: Uuid,
    /// Seems to always be MCPE
    server_type: String,
    /// First line in the Friends tab list item
    pub title: String,
    /// Second line in the Friends tab list item
    pub description: String,
    /// The string must be in format X.X.X, where X is a number, otherwise the client wont display it.
    /// See [Bedrock version numbers](https://wiki.vg/Bedrock_Protocol_version_numbers)
    pub protocol: (usize, String),
    /// Not displayed in the friends tab as of MCBE v1.19.51
    pub gamemode: Gamemode,
    /// Server's IPv4 port
    pub port4: u16,
    /// Server's IPv6 port
    pub port6: u16,
}

/// Fills the Pong packet with default values for MCBE Dedicated server v1.19.51
impl Default for Pong {
    fn default() -> Self {
        Self {
            init_time: Instant::now(),
            guid: rand::thread_rng().gen(),
            server_type: "MCPE".to_string(),
            title: "Minecraft server".to_string(),
            description: "Bedrock level".to_string(),
            protocol: (560, "1.19.51".to_string()),
            gamemode: Gamemode::Creative,
            port4: 19132,
            port6: 19133,
        }
    }
}

/// Takes everything it can from server.properties and the rest is set
/// using [`Default::default()`]
impl From<ServerProperties> for Pong {
    fn from(props: ServerProperties) -> Self {
        Self {
            title: props.server_name,
            description: props.level_name,
            gamemode: props.gamemode,
            port4: props.port4,
            port6: props.port6,
            ..Default::default()
        }
    }
}

impl Pong {
    /// Renders a Pong packet that can be sent over UDP
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut msg = Vec::<u8>::with_capacity(120);

        msg.push(PONG);

        let timestamp = self.init_time.elapsed().as_millis();
        msg.append(&mut timestamp.to_be_bytes()[8..].to_vec());

        msg.append(&mut self.guid.to_be_bytes().to_vec());
        msg.append(&mut MAGIC.to_vec());

        let descriptor = format!(
                "{server_type};{title};{version_number};{version_str};0;1;999;{description};{gamemode};1;{p4};{p6};0;",
                server_type = self.server_type,
                title = self.title,
                version_number = self.protocol.0,
                version_str = self.protocol.1,
                description = self.description,
                gamemode = self.gamemode.to_string(),
                p4 = self.port4,
                p6 = self.port6,
            );
        msg.append(&mut (descriptor.len() as StringLength).to_be_bytes().to_vec());
        msg.append(&mut descriptor.as_bytes().to_vec());
        msg
    }
}
