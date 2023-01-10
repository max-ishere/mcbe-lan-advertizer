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
//! # Modes of operation
//!
//! ## Relay
//!
//! In relay mode the program will listen on port 19132 and forward packets
//! to the real server. This way the server thinks the Ping comes from a
//! localhost client and should not deny this request. Then the program will
//! take the Pong that the real server generated and send it to the client.
//!
//! # server.properties
//!
//! In properties mode the program will read `server.properties` and use the
//! information there to generate a Pong packet. This can be used if the
//! server still does not respond with Pong requests.
//!
//! Because the actual server runs on a random port that port number is
//! transmitted in the Pong packet. Thus we can run a dedicated server on
//! any port and point this program to Pong the correct, but arbitrary port.
//!
//! # Usage
//!
//! See [`config::args`]
//!
//! # Still have questions or need support?
//!
//! Read the extended [README.md](https://github.com/max-ishere/mcbe-lan-advertizer) on GitHub. It has more details while this page
//! is dedicated to developers.

#[macro_use]
extern crate thiserror;

pub mod config;
pub mod error;
pub mod pong;
