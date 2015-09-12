extern crate ease;

use ease::{Url, Request};

fn main() {
    // simple
    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{}", Request::new(url).param("foo", "bar").get().unwrap().body);

    // multiple
    let url = Url::parse("http://httpbin.org/get").unwrap();
    let value = "1729".to_owned();
    let mut req = Request::new(url);
    req.param("foo", &value);
    req.param("bar", &value);
    println!("{}", req.get().unwrap().body);
}
