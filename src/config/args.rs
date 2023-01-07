//! CLI Argument parser
//!
//! The [`Args`] struct represents the CLI args. This is where [`Mode`]s of operation are defined.

use clap::{Parser, Subcommand};

/// Run the LAN Advertizer in one of modes below
#[derive(Parser)]
pub struct Args {
    /// Specifies how the relay should run
    #[clap(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand)]
pub enum Mode {
    /// Relays the Ping to the real server
    Relay {
        /// Path to server.properties. Default: `server.properties` in current directory.
        #[clap(value_parser, default_value = "server.properties")]
        props: String,
    },
    /// Reponds with information found in server.properties.
    /// You MUST specify server protocol with proto subcommand
    FromProps {
        /// Path to server.properties. Default: `server.properties` in current directory.
        #[clap(value_parser, default_value = "server.properties")]
        props: String,

        #[clap(subcommand)]
        proto: Proto,
    },
}

// I really wanted to have `--proto 560 1.19.51`. However at this time tuple arguments are not
// supported so I made it an only subcommand that is also required. Looks a bit confusing to the
// user but because there is an optional props arg before proto this seems like the best trade of
#[derive(Subcommand)]
pub enum Proto {
    /// REQUIRED Sets the server protocol version. See <https://wiki.vg/Bedrock_Protocol_version_numbers>
    Proto {
        /// Protocol version number as single number. E.g: `560`
        numeric: u64,
        /// The regular Minecraft version. E.g: `1.19.51`
        string: String,
    },
}
