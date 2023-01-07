//! Parses `server.properties` file
//!
//! [`ServerProperties`] specifies which fields are parsed from the file. Other fields are ignored.
//!
//! Since at this time threre are only 3 valid gamemodes in Minecraft, the gamemode is parsed into
//! an enum [`Gamemode`].

use std::{collections::HashMap, str::FromStr};

use anyhow::Context;

use crate::error::{BadGamemode, ContainsSemicolon};

/// Use [`ServerProperties::from_str()`] to parse the file.
pub struct ServerProperties {
    pub server_name: String,
    pub level_name: String,
    pub gamemode: Gamemode,
    pub max_players: usize,
    pub port4: u16,
    pub port6: u16,
}

/// Use `from_str` and `to_string` to convert between [`Gamemode`] and [`String`].
#[derive(Clone, Default)]
pub enum Gamemode {
    Survival,
    #[default]
    Creative,
    Adventure,
}

impl FromStr for Gamemode {
    type Err = BadGamemode;

    fn from_str(mode: &str) -> Result<Self, Self::Err> {
        use Gamemode::*;

        Ok(match mode {
            "creative" => Creative,
            "survival" => Survival,
            "adventure" => Adventure,
            other => return Err(BadGamemode(other.to_string())),
        })
    }
}

impl ToString for Gamemode {
    fn to_string(&self) -> String {
        use Gamemode::*;

        match self {
            Survival => "survival",
            Creative => "creative",
            Adventure => "adventure",
        }
        .to_string()
    }
}

impl FromStr for ServerProperties {
    type Err = anyhow::Error;

    fn from_str(config: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::<String, String>::new();

        // Collect all key value pairs
        for line in config.lines() {
            let not_comment = &line[0..line.find('#').unwrap_or(line.len())];

            let equals = match not_comment.find('=') {
                Some(i) => i,
                None => continue,
            };

            let (key, value) = not_comment.split_at(equals);
            map.insert(key.trim().to_owned(), value.trim()[1..].to_owned());
        }

        // Extract the useful keys
        let mut get =
            move |key, default: &str| map.remove(key).unwrap_or_else(|| default.to_string());
        let sanitize = |name: String| match name.contains(';') {
            false => Ok(name),
            true => Err(ContainsSemicolon(name)),
        };

        Ok(ServerProperties {
            server_name: sanitize(get("server-name", "Minecraft Server"))
                .with_context(|| "Invalid server name in server.properties")?,
            level_name: sanitize(get("level-name", "Bedrock server"))
                .with_context(|| "Invalid server name in server.properties")?,

            gamemode: Gamemode::from_str(&get("gamemode", ""))
                .with_context(|| "Invalid gamemode in server.properties.")?,
            max_players: get("max-players", "1")
                .parse()
                .with_context(|| "Invalid player limit in server.properties")?,
            port4: get("server-port", "")
                .parse()
                .with_context(|| "Invalid Server IP v4 port.")?,
            port6: get("server-portv6", "")
                .parse()
                .with_context(|| "Invalid Server IP v6 port.")?,
        })
    }
}
