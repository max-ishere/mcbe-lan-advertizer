//! # Minecraft Bedrock Server LAN Advertizer
//!
//! On pocket edition there is a way to play with friends on same LAN using
//! the friends tab. For this to work a client will send a Ping packet as a
//! broadcast to port 19132. Normally a server will respond to these packets
//! with Pong packet.
//!
//! Most mobile servers (open to LAN games) will respond to Pings properly.
//! However a dedicated server may ignore a request because it thinks it's
//! outside the LAN. Or because of some other reason that we do not know.
//!
//! This program has 2 modes of operation: *Relay* and *From Properties*.
//! In relay mode the program will listen on port 19132 and forward packets
//! to the real server. This way the server thinks the Ping comes from a
//! localhost client and should not deny this request. Then the program will
//! take the Pong that the real server generated and send it to the client.
//!
//! In properties mode the program will read `server.properties` and use the
//! information there to generate a Pong packet. This can be used if the
//! server still does not respond with Pong requests.
//!
//! The intended use for Ping/Pong in Bedrock Edition is to allow a mobile
//! device to open a random port UDP socket. This enhanses security of the
//! device slightly. And the 19132 port socket runs a simpler Ping/Pong code
//! that in theory should not have security vulnerabilities.
//!
//! Because the actual server runs on a random port that port number is
//! transmitted in the Pong packet. Thus we can run a dedicated server on
//! any port and point this program to Pong the correct, but arbitrary port.
//!
//! # Instalation
//!
//! ## Using `cargo`
//!
//! ```shell
//! cargo install mcbe-lan-advertizer
//! ```
//!
//! ## Build from source
//!
//! **Prerequisites**:
//!
//! - `cargo` and rust installed
//! - `git` installed
//!
//! **Process:**
//!
//! Replace `PATH_TO_BEDROCK_SERVER` with where your server is located. If you
//! run multiple servers you can copy the executable to a directory above
//! those.
//!
//! ```shell
//! git clone "https://github.com/max-ishere/mcbe-lan-advertizer"
//! cd mcbe-lan-advertizer
//! cargo build --profile release
//! cp target/release/advertize PATH_TO_BEDROCK_SERVER
//! ```
//!
//! Optionaly remove the git repository with source code:
//!
//! ```shell
//! cd ..
//! rm -rf mcbe-lan-advertizer
//! ```
//!
//! # Usage
//!
//! First run your server on any port other than 19132. This is because that
//! port will be used by this program.
//!
//! This is a terminal application meaning you need to open a terminal and run
//! commands there. If you used cargo to install then you can run the app using
//! `avertize`. If you built it from source then run `./advertize` in the
//! directory where the executable file is located. The instructions below assume
//! you used `cargo install`.
//!
//! ## Getting help
//!
//! ```shell
//! advertize help
//! ```
//!
//! ## Relay mode
//!
//! ```shell
//! advertize relay [ server.properties ]
//! ```
//!
//! Where:
//! - `server.properties` (optional) path to server.properties file for the
//! currently running server.
//!
//! ## From Properties mode
//!
//! ```shell
//! advertize from-props [ server.properties ] proto <numeric> <mc_version>
//! ```
//!
//! Where:
//! - `server.properties` (optional) path to server.properties file for the
//! currently running server.
//! - `numeric`, `mc_version`: use [Protocol version numbers](https://wiki.vg/Bedrock_Protocol_version_numbers)
//! to fill this in.
//!
//! # Still have questions or need support?
//!
//! Ask them on [Github Issues](https://github.com/max-ishere/mcbe-lan-advertizer/issues).

#[macro_use]
extern crate thiserror;

pub mod config;
pub mod error;
pub mod pong;
