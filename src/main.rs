#[macro_use]
extern crate clap;
use clap::App;

mod client;
mod string;
mod params;
mod request;

use params::Params;

fn main() {
    let yaml = load_yaml!("../config/cli.yml");
    let matches = App::from(yaml).get_matches();

    let params =
        if let Some(config_file) = matches.value_of("config") {
            println!("load {} to get params", config_file);
            Params::new(
                "Error",
                "email@email.com",
                "123123123",
                "91 123 12 12",
                "Address",
                "1234-123",
                "Lisbon",
                1,
                1,
                2020,
                21, 00,
                22, 00,
                )
        } else {
            Params::new(
                "Error",
                "email@email.com",
                "123123123",
                "91 123 12 12",
                "Address",
                "1234-123",
                "Lisbon",
                1,
                1,
                2020,
                21, 00,
                22, 00,
                )
        };

        client::run(&params);
}
