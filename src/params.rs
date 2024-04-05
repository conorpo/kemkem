/*
    Loads the correct parameters for the corresponding security level
*/

use serde::Deserialize;

pub const N : usize = 256;
pub const Q : u16 = 3329;

#[derive(Deserialize, Debug)]
pub struct MlKemParams {
    k: u8,
    eta_1: u8,
    eta_2: u8,
    d_u: u8,
    d_v: u8,
}

const PARAMS_DIR: &str = "./config/";

impl MlKemParams {
    pub fn get() -> Self {
        let args: Vec<String> = std::env::args().collect();

        let config_path = PARAMS_DIR.to_string() + (if args.len() > 1 {
            &args[1]
        } else {
            "ML-KEM-512.toml"
        });

        let config_str = std::fs::read_to_string(config_path).expect("Failed to read config file");

        toml::from_str(&config_str).expect("Failed to parse config file")
    }
}