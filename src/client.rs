use crate::params::Params;
use crate::request::Request;
use scraper::{Html, Selector, ElementRef};
use ureq::IpVersion;

pub fn run(params: &Params) {
    let url = format!("https://www.estadio.ulisboa.pt/webform/pedido-de-reserva-de-espacos");

    // get webpage
    let webpage = ureq::get(&url)
        .set_preferred_ip_version(IpVersion::V4)
        .call()
        .into_string()
        .unwrap();

    let document = Html::parse_document(&webpage);

    let selector = Selector::parse(".webform-client-form").unwrap();
    let webform = document.select(&selector).next().unwrap();

    // extract needed form IDs
    let form_id = element_value(&webform, "input[name=\"form_id\"]");
    let form_build_id = element_value(&webform, "input[name=\"form_build_id\"]");

    // perform request
    let mut request = Request::new(params);
    request.form_id = Some(form_id);
    request.form_build_id = Some(form_build_id);

    let urlencoded = serde_qs::to_string(&request).unwrap();

    let response = ureq::post(&url)
        .set_preferred_ip_version(IpVersion::V4)
        .set("User-Agent", "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0")
        .set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .set("Accept-Language", "en-US,en;q=0.5' --compressed")
        .set("Content-Type", "application/x-www-form-urlencoded")
        .set("Origin", "https://www.estadio.ulisboa.pt")
        .set("DNT", "1")
        .set("Connection", "keep-alive")
        .set("Referer", "https://www.estadio.ulisboa.pt/webform/pedido-de-reserva-de-espacos")
        .set("Cookie", "has_js=1")
        .set("Upgrade-Insecure-Requests", "1")
        .send_string(&urlencoded);

    // analyse response
    if response.ok() {
        println!("[SUCCESS] Form Submission Successful. You should receive a confirmation e-mail in the inbox for {}.", params.email);
    } else {
        println!("[ERROR] Form Submission Unsuccessful. Please try again and consider changing the values you assigned.");
    }
}

fn element_value(document: &ElementRef, css_selector: &str) -> String {
    let selector = Selector::parse(css_selector).unwrap();
    let element = document.select(&selector).next().unwrap();
    element.value().attr("value").unwrap().replace("\"", "")
}
