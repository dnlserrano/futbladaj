extern crate serde_json;

use crate::params::Params;
use crate::request::Request;

pub fn run(params: &Params) -> i32 {
    let request = Request::new(params);
    let serialized = serde_json::to_string(&request).unwrap();
    println!("serialized = {}", serialized);
    42
}
