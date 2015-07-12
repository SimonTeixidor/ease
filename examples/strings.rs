extern crate ease;

use ease::{Url, RestClient};

fn main() {
    let url = Url::parse("http://httpbin.org/get").unwrap();
    println!("{}",
             RestClient::new(url)
                        .get()
                        .unwrap()
            );

    println!("\n");

    let url = Url::parse("http://httpbin.org/post").unwrap();
    println!("{}",
             RestClient::new(url)
                        .param(("foo", "bar"))
                        .post()
                        .unwrap()
            );

    let url = Url::parse("http://httpbin.org/put").unwrap();
    println!("{}",
             RestClient::new(url)
                        .param(("foo", "bar"))
                        .put()
                        .unwrap()
            );

    let url = Url::parse("http://httpbin.org/delete").unwrap();
    println!("{}",
             RestClient::new(url)
                        .param(("foo", "bar"))
                        .delete()
                        .unwrap()
            );
}
