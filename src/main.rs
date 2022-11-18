//! This app is meant to be run on the same computer that runs a dedicated server.
//! The app will respond to broadcasts from minecraft clients and tell them the
//! port for your dedicated server.
//!
//! If you have any problems with this app submit to
//! TODO: github issues page
//!
//! # Running
//!
//! This app can run without configuration but consider passing CLI args or creating lan_advertize.toml.
//! Keep in mind that after only one configuration source can be used. This means that if you use CLI args
//! they have to customize everything you need to be custom. Undefined settings will be set to default
//!
//! # Config lookup priority
//!
//! - CLI args
//! - lan_advertize.toml in current directory
//! - lan_advertize.toml in:
//!   - Linux, MacOS: `~/.config`
//!   - Windows: `C:/Users/$USER/AppData/Roaming/`

#[macro_use]
extern crate derive_builder;

use raknet::PongBuilder;
use std::net::UdpSocket;
// 0000   1c 00 00 00 00 00 29 7f 1e 92 45 a8 8e 41 05 e5
// 0010   dd 00 ff ff 00 fe fe fe fe fd fd fd fd 12 34 56
// 0020   78 00 5b 4d 43 50 45 3b 6d 61 78 20 69 73 68 65
// 0030   72 65 33 35 35 31 3b 35 35 37 3b 31 2e 31 39 2e
// 0040   34 31 3b 31 3b 35 3b 31 32 30 33 32 31 39 36 39
// 0050   31 32 36 32 31 39 38 36 38 32 39 3b 4d 79 20 77
// 0060   6f 72 6c 64 3b 53 75 72 76 69 76 61 6c 3b 31 3b
// 0070   36 31 35 34 33 3b 36 31 35 34 34 3b 30 3b

pub mod config;

pub mod raknet {
    use rand::Rng;
    use std::{
        net::{SocketAddr, UdpSocket},
        time::Instant,
    };

    const PONG: u8 = 0x1c;
    type Uuid = u64;
    const MAGIC: [u8; 16] = [
        0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56,
        0x78,
    ];
    type StringLength = u16;

    /// Use [`PongBuilder`] to initialize this struct. See [`Pong::repond()`] for this struct's usecase.
    #[derive(Builder)]
    pub struct Pong {
        /// Raknet timer field. Seems like its used for tracking packet order
        ///
        /// # Default
        /// ```
        /// Instant::now()
        /// ```
        #[builder(setter(skip), default = "Instant::now()")]
        init_time: Instant,

        /// Server GUID. Likely a random number will suffice.
        ///
        /// # Default
        ///
        /// A random number
        #[builder(default = "rand::thread_rng().gen::<u64>()")]
        guid: Uuid,

        /// A string similar to User-Agent in a browser.
        ///
        /// # Default
        ///
        /// `"MCPE"`
        #[builder(default = r#""MCPE".to_string()"#)]
        server_type: String,

        /// Bold title text on the button. Keep is short and alphanumeric.
        #[builder(default = r#""LAN Server".to_string()"#)]
        title: String,

        /// Dim text below the [`Self::title`].
        #[builder(default = r#""Edit lan_advertize.yaml".to_string()"#)]
        description: String,

        /// See [wiki.vg](https://wiki.vg/Bedrock_Protocol_version_numbers) or [Minecraft fandom wiki](https://minecraft.fandom.com/wiki/Protocol_version#Bedrock_Edition_2) for your minecraft version protocol.
        /// If none of the above are working links then [google it](https://www.google.com/search?hl=en&q=bedrock%20protocol%20version%20number)
        ///
        /// # Example
        /// ```
        /// (557, "1.19.41".to_string)
        /// ```
        protocol: (usize, String),

        /// You should stick to "Survival" and "Creative" but you can also experiment with custom values if you want to.
        /// They may not work though
        #[builder(setter(into))]
        gamemode: String,

        /// IP v4 server port on localhost
        #[builder(default = "19133")]
        port_v4: u16,

        /// IP v6 server port on localhost
        #[builder(default = "19134")]
        port_v6: u16,
    }
    // TODO: impl repond() for pong
    impl Pong {
        pub fn respond(&self, socket: &UdpSocket, addr: &SocketAddr) -> std::io::Result<()> {
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
                gamemode = self.gamemode,
                p4 = self.port_v4,
                p6 = self.port_v6,
            );
            msg.append(&mut (descriptor.len() as StringLength).to_be_bytes().to_vec());
            msg.append(&mut descriptor.as_bytes().to_vec());
            match socket.send_to(&msg, addr) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    {
        println!("Ready to pong on port: {}", 19132);
        let mut buf = [0u8; 1000];
        let pong = PongBuilder::default()
            .protocol((547, "1.19.41".to_string()))
            .gamemode("Dedication!")
            .build()
            .unwrap();
        loop {
            let socket = UdpSocket::bind(("0.0.0.0", 19132))?;
            let (len, addr) = socket.recv_from(&mut buf)?;

            // TODO: Verify its actually a minecraft packet
            let buf = &mut buf[..len];

            println!("{addr}: Pinged me");

            pong.respond(&socket, &addr)?;
        }
    }
}

pub struct Servers {
    list: Vec<LanAdvert>,
}

pub struct LanAdvert {
    title: String,
    description: String,
    server_type: String,
    protocol: Protocol,
    guid: u64,
    server_id: u128,
    gamemode: String,
    port: Port,
}

pub struct Protocol {
    numeric: usize,
    display: String,
}

pub struct Port {
    v4: u16,
    v6: u16,
}
