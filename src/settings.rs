use graphics::types::Color;

use toml;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;


pub struct Settings {
    colors: Colors,
}

pub struct Colors {
    background_color:     Color,
    foreground_color:     Color,
    player_color:         Color,
    player_outline_color: Color,
}

#[derive(Deserialize)]
struct RawSettings {
    colors: RawColors,
}

#[derive(Deserialize)]
struct RawColors {
    background_color:     String,
    foreground_color:     String,
    player_color:         String,
    player_outline_color: String,
}

#[derive(Debug)]
pub enum SettingsError {
    Io(String),
    De(String),
    Parse(String),
}


impl Settings {
    pub fn get_from<P: AsRef<Path>>(
        settings_path: P
    ) -> Result<Self, SettingsError> {
        let mut settings_file = match File::open(settings_path) {
            Ok(f)  => f,
            Err(e) => return Err(
                SettingsError::Io(e.description().to_string())
            ),
        };
        let mut contents = String::new();
        if let Err(e) = settings_file.read_to_string(&mut contents) {
            return Err(SettingsError::Io(e.description().to_string()));
        }

        match toml::from_str(&contents) {
            Ok(s)  => Self::unraw(s),
            Err(e) => Err(SettingsError::De(e.description().to_string())),
        }
    }

    fn unraw(raw: RawSettings) -> Result<Self, SettingsError> {
        let background_color = hex_to_color(&raw.background_color)?;
        let foreground_color = hex_to_color(&raw.foreground_color)?;
        let player_color = hex_to_color(&raw.player_color)?;
        let player_outline_color = hex_to_color(&raw.player_outline_color)?;

        Ok(Settings {
            background_color,
            foreground_color,
            player_color,
            player_outline_color,
        })
    }
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &SettingsError::Io(ref msg) => write!(f, "I/O error: {}", msg),
            &SettingsError::De(ref msg) => write!(
                f,
                "Deserialization error: {}",
                msg
            ),
            &SettingsError::Parse(ref msg) => write!(
                f,
                "Raw settings parse error: {}",
                msg
            ),
        }
    }
}

pub fn hex_to_color(hex_str: &str) -> Result<Color, SettingsError> {
    match u32::from_str_radix(&hex_str[1..], 16) {
        Ok(parsed_int) => if parsed_int > 0xFFFFFF {
            let r = parsed_int >> 24 & 0xFF;
            let g = parsed_int >> 16 & 0xFF;
            let b = parsed_int >> 8  & 0xFF;
            let a = parsed_int       & 0xFF;

            Ok([r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                a as f32 / 255.0])
        } else {
            let r = parsed_int >> 16 & 0xFF;
            let g = parsed_int >> 8  & 0xFF;
            let b = parsed_int       & 0xFF;

            Ok([r as f32 / 255.0,
                g as f32 / 255.0,
                b as f32 / 255.0,
                1.0])
        },
        Err(e) => Err(SettingsError::Parse(e.description().to_string())),
    }
}
