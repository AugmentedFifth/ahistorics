use graphics::types::Color;

use toml;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;


#[derive(Deserialize)]
pub struct Settings {
    background_color:     Color,
    foreground_color:     Color,
    player_color:         Color,
    player_outline_color: Color,
}


impl Settings {
    pub fn get_from<P: AsRef<Path>, E: From<Error>>(settings_path: P) -> Result<Self, E> {
        let mut settings_file = File::open(settings_path)?;
        let mut contents = String::new();
        settings_file.read_to_string(&mut contents)?;

        toml::from_str(contents)
    }
}
