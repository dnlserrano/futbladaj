#[macro_use]
extern crate clap;
use clap::App;

extern crate serde_yaml;

use std::fs::File;
use std::io::prelude::*;

use params::Params;

mod client;
mod string;
mod params;
mod request;

fn main() {
    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from(yaml).get_matches();
    let username = "Daniel Serrano".to_string();

    let params =
        if let Some(config_file) = matches.value_of("config") {
            let mut file = File::open(config_file).unwrap();
            let mut yaml = String::new();
            file.read_to_string(&mut yaml).unwrap();

            serde_yaml::from_str(&yaml).unwrap()
        } else {
            default_params(username)
        };

    client::run(&params);
}

fn default_params(username: String) -> Params {
    Params::new(
        username,
        "email@email.com".to_string(),
        "123123123".to_string(),
        "91 123 12 12".to_string(),
        "Address".to_string(),
        "1234-123".to_string(),
        "Lisbon".to_string(),
        1,
        1,
        2020,
        21, 00,
        22, 00,
        )
}
