extern crate ureq;

pub fn run() {
    let url = format!("localhost:4567");
    println!("url = {:?}", url);

    let json = {};
    println!("json = {:?}", json);

    let response = ureq::post(&url)
        .set("X-My-Header", "Secret")
        .send_json(json!(json));
    println!("response = {:?}", response);
}
