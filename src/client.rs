extern crate serde_qs;
extern crate ureq;

use crate::params::Params;
use crate::request::Request;

pub fn run(params: &Params) {
    let url = format!("http://localhost:4567");

    let request = Request::new(params);
    let urlencoded = serde_qs::to_string(&request).unwrap();

    let response = ureq::post(&url)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0")
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .set("Accept-Language", "en-US,en;q=0.5' --compressed")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set("Origin", "https://www.estadio.ulisboa.pt")
        .set("DNT", "1")
        .set("Referer", "https://www.estadio.ulisboa.pt/webform/pedido-de-reserva-de-espacos")
        .send_string(&urlencoded);

    if response.ok() {
        println!("[SUCCESS] form submission successful");
    } else {
        println!("[ERROR] form submission unsuccessful");
    }
}
