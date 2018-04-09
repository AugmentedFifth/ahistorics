use failure::Error;
use graphics::types::Color;
use toml;
use std::{ffi::OsString, fs::File, io::Read, path::{Path, PathBuf}};


#[derive(Debug, Clone)]
pub struct Settings {
    pub colors: Colors,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Fail)]
pub enum SettingsError {
    #[fail(display = "{:?} is a malformed path that doesn't refer to any \
                      file name", path)]
    BadFilePath {
        path: PathBuf,
    },
    #[fail(display = "No file with the name {:?} found in the specified path \
                      nor any of its parents/ancestors", filename)]
    FileNotFound {
        filename: OsString,
    },
}


impl Settings {
    pub fn get_from<P: AsRef<Path>>(settings_path: P) -> Result<Self, Error> {
        let mut settings_file = File::open(settings_path)?;
        let mut contents = String::new();
        settings_file.read_to_string(&mut contents)?;

        Self::unraw(&toml::from_str(&contents)?)
    }

    pub fn get_from_recur<P>(settings_path: P) -> Result<Self, Error>
        where P: AsRef<Path>
    {
        let path = settings_path.as_ref();
        if let Ok(s) = Self::get_from(path) {
            return Ok(s);
        }

        let filename = path.file_name().ok_or(SettingsError::BadFilePath {
            path: path.to_owned(),
        })?;

        let canonical_path = path.parent().ok_or(SettingsError::FileNotFound {
            filename: filename.to_owned(),
        })?;
        let canonical_path_buf = canonical_path.canonicalize()?;
        let mut canonical_path = canonical_path_buf.as_path();

        while let Some(p) = canonical_path.parent() {
            if let Ok(s) = Self::get_from(p.join(filename)) {
                return Ok(s);
            }

            canonical_path = p;
        }

        Err(SettingsError::FileNotFound {
            filename: filename.to_owned(),
        }.into())
    }

    fn unraw(raw: &RawSettings) -> Result<Self, Error> {
        let background_color = hex_to_color(&raw.colors.background_color)?;
        let foreground_color = hex_to_color(&raw.colors.foreground_color)?;
        let player_color = hex_to_color(&raw.colors.player_color)?;
        let player_outline_color =
            hex_to_color(&raw.colors.player_outline_color)?;

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

pub fn hex_to_color(hex_str: &str) -> Result<Color, Error> {
    let parsed_int = u32::from_str_radix(&hex_str[1..], 16)?;
    if parsed_int > 0xFF_FF_FF {
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
    }
}
