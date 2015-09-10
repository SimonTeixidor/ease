Ease - HTTP clients for Rust [![Build Status](https://travis-ci.org/SimonPersson/ease.png?branch=master)](https://travis-ci.org/SimonPersson/ease)
=================================

`Ease` is a library for interacting with RESTful APIs.

Examples
========

Make a GET call and print the result:
```rust
extern crate ease;

use ease::{Url, Request};

fn main() {
    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{}", Request::new(url).param("foo", "bar").get().unwrap().body);
}
```

Responses can also be deserialized from JSON, see `examples/json` for an
example.

[Documentation](http://simonpersson.github.io/ease/)
====================================================

Documentation is available online and can be built with `cargo doc`
for a local copy.

License
=======

MIT
