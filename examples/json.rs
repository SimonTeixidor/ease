#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate ease;

use std::collections::HashMap;
use ease::RestClient;

#[derive(Deserialize, Debug)]
struct Response {
    args: HashMap<String, String>,
    data: Option<String>,
    files: HashMap<String, String>,
    form: HashMap<String, String>,
    headers: HashMap<String, String>,
    json: Option<String>,
    origin: String,
    url: String
}

fn main() {
    println!("{:?}",
             RestClient::new("http://httpbin.org/post")
                        .ok().expect("Couldn't parse URL.")
                        .post_json_as::<Response>()
        );
    
    println!("{:?}",
             RestClient::new("http://httpbin.org/get")
                        .ok().expect("Couldn't parse URL.")
                        .get_json_as::<Response>()
        );
}
