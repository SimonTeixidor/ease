Ease - HTTP clients for Rust [![Build Status](https://travis-ci.org/SimonPersson/ease.png?branch=master)](https://travis-ci.org/SimonPersson/ease)
=================================

`Ease` is a library for interacting with RESTful APIs.

Examples
========

In `Cargo.toml`, put:
```toml
[dependencies]
ease = "*"
```

Make a GET request and print the result:
```rust
extern crate ease;

use ease::{Url, Request};

fn main() {
    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{}", Request::new(url).param("foo", "bar").get().unwrap().body);
}
```

Make a POST request and deserialize the response from JSON using
[serde](https://github.com/serde-rs/serde):
```rust
#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate ease;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use ease::{Url, Request};

#[derive(Deserialize, Debug)]
struct PostResponse {
    args: HashMap<String, String>,
    data: Option<String>,
    files: Option<HashMap<String, String>>,
    form: Option<HashMap<String, String>>,
    headers: HashMap<String, String>,
    json: Option<String>,
    origin: String,
    url: String,
}

fn main() {
    let url = Url::parse("http://httpbin.org/post").unwrap();
    println!("{:#?}", Request::new(url).post().and_then(|res| res.from_json::<PostResponse>()));
}
```

[Documentation](http://simonpersson.github.io/ease/)
====================================================

Documentation is available online and can be built with `cargo doc`
for a local copy.

License
=======

MIT

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
