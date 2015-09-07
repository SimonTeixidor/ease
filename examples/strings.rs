extern crate ease;

use ease::{Url, Request};

fn main() {
    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{}", Request::new(url).param("foo", "bar").get().unwrap().body);
}
