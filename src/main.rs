#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};

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

    let params =
        if let Some(config_file) = matches.value_of("config") {
            let mut file = File::open(config_file).unwrap();
            let mut yaml = String::new();
            file.read_to_string(&mut yaml).unwrap();

            serde_yaml::from_str(&yaml).unwrap()
        } else {
            argmatches_to_params(&matches)
        };

    client::run(&params);
}

fn argmatches_to_params(matches: &ArgMatches) -> Params {
    let username = to_string(matches, "username");
    let email = to_string(matches, "email");
    let fiscal_number = to_string(matches, "fiscal_number");
    let phone = to_string(matches, "phone");
    let address = to_string(matches, "address");
    let postcode = to_string(matches, "postcode");
    let day = to_i32(matches, "day");
    let month = to_i32(matches, "month");
    let year = to_i32(matches, "year");
    let start_hour = to_i32(matches, "start_hour");
    let end_hour = to_i32(matches, "end_hour");

    Params::new(
        username, email, fiscal_number, phone, address, postcode,
        day, month, year, start_hour, end_hour
        )
}

fn to_string(matches: &ArgMatches, arg_name: &str) -> String {
    matches.value_of(arg_name).unwrap().to_string()
}

fn to_i32(matches: &ArgMatches, arg_name: &str) -> i32 {
    matches.value_of(arg_name).unwrap().to_string().parse::<i32>().unwrap()
}
