#[macro_use]
extern crate serde_derive;

extern crate ease;

use std::collections::HashMap;
use ease::{Url, Request};

#[derive(Deserialize, Debug)]
struct Response {
    cookies: HashMap<String, String>,
}

fn main() {
    let url = Url::parse("http://httpbin.org/cookies/set?key=val&key2=val2").unwrap();
    if let Ok(cookies) = Request::new(url)
        .get()
        .map(|res| res.get_cookies()) {
        let url = Url::parse("http://httpbin.org/cookies").unwrap();
        println!("{:#?}",
                 Request::new(url)
                     .cookies(cookies)
                     .get()
                     .map(|res| res.from_json::<Response>()));
    } else {
        println!("Couldn't set cookies :(");
    }
}
