use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
pub struct Config {
    pub magnets: Vec<Magnet>,
    pub max_distance: f64,
    pub magnet_strength: f64,
}

#[derive(Deserialize)]
pub struct Magnet {
    pub frequency: usize,
    pub position: (f64, f64),
}

pub fn load_config() -> Config {
    let f = File::open(&"config.ron").expect("Failed to open config file");
    let config: Config = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    config
}
