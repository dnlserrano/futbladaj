#[macro_use]
extern crate ureq;

#[macro_use]
extern crate clap;
use clap::App;

mod cli;
mod client;
mod string;
mod params;
mod request;

use params::Params;

fn main() {
    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(config_file) = matches.value_of("config") {
        config::run(config_file);
    } else {
        let params = Params::new("Daniel Serrano");
        cli::run(&params);
    }
}
