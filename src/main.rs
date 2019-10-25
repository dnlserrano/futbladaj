#[macro_use]
extern crate clap;
use clap::App;

mod config;
mod cli;

fn main() {
    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(config_file) = matches.value_of("config") {
        config::run(config_file);
    } else {
        cli::run();
    }
}
