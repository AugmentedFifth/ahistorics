use graphics::types::Color;

use toml;

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;


pub struct Settings {
    pub colors: Colors,
}

pub struct Colors {
    pub background_color:     Color,
    pub foreground_color:     Color,
    pub player_color:         Color,
    pub player_outline_color: Color,
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

    pub fn get_from_recur<P: AsRef<Path>>(
        settings_path: P
    ) -> Result<Self, SettingsError> {
        let path = settings_path.as_ref();
        if let Ok(s) = Self::get_from(path) {
            return Ok(s);
        }

        let filename = if let Some(f) = path.file_name() {
            f
        } else {
            return Err(SettingsError::Io(format!(
                "{:?} is a malformed path that doesn't refer \
                 to any file name.",
                path
            )));
        };

        let canonical_path = if let Some(cp) = path.parent() {
            cp
        } else {
            return Err(SettingsError::Io(format!(
                "No file with the name {:?} found in the specified path \
                or any of its parents/ancestors.",
                filename
            )));
        };
        let canonical_path_buf = match canonical_path.canonicalize() {
            Ok(cp) => cp,
            Err(e) => return Err(SettingsError::Io(
                e.description().to_string()
            )),
        };
        let mut canonical_path = canonical_path_buf.as_path();

        while let Some(p) = canonical_path.parent() {
            if let Ok(s) = Self::get_from(p.join(filename)) {
                return Ok(s);
            }

            canonical_path = p;
        }

        Err(SettingsError::Io(format!(
            "No file with the name {:?} found in the specified path \
             or any of its parents/ancestors.",
            filename
        )))
    }

    fn unraw(raw: RawSettings) -> Result<Self, SettingsError> {
        let background_color = hex_to_color(&raw.colors.background_color)?;
        let foreground_color = hex_to_color(&raw.colors.foreground_color)?;
        let player_color = hex_to_color(&raw.colors.player_color)?;
        let player_outline_color = hex_to_color(&raw.colors.player_outline_color)?;

        let colors = Colors {
            background_color,
            foreground_color,
            player_color,
            player_outline_color,
        };

        Ok(Settings {
            colors,
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
