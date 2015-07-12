Ease - Write REST clients in Rust [![Build Status](https://travis-ci.org/SimonPersson/ease.png?branch=master)](https://travis-ci.org/SimonPersson/ease)
=================================

Ease simplifies the task of writing REST clients. Requests
are constructed with an easy to use builder pattern and JSON
responses can be automatically deserialised into a matching
struct.

The library build on both stable and nightly rust. However,
deserialising JSON is a bit awkward without featurs from nightly,
see [Serde](https://github.com/serde-rs/serde#serialization-without-macros)
for information about that.

Examples
========

Make a GET call and print the result:
```rust
extern crate ease;

use ease::{Url, RestClient};

fn main() {
    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{}",
             RestClient::new(url)
                        .get()
                        .unwrap()
            );
}
```

Make a POST call, parse the JSON reply in a struct, and print the struct:
```rust
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate ease;

use std::collections::HashMap;
use ease::{Url, RestClient};

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
    let url = Url::parse("http://httpbin.org/post").unwrap();
    println!("{:?}",
             RestClient::new(url)
                        .post_json_as::<Response>()
        );
}
```

License
=======

MIT
