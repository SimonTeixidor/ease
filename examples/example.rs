
extern crate ease;

use ease::Client;
use ease::UserAgent;

fn main() {
    println!("{}", 
             Client::new("https://api.discogs.com/releases/249504")
                        .ok().expect("Couldn't parse URL.")
                        .header(
                            UserAgent("FooBarApp/2.0".to_owned())
                            )
                        .get()
                        .unwrap()
            );

    println!("\n");

    println!("{}", 
             Client::new("http://httpbin.org/post")
                        .ok().expect("Couldn't parse URL.")
                        .header(
                            UserAgent("FooBarApp/2.0".to_owned())
                            )
                        .param(("foo", "bar"))
                        .post()
                        .unwrap()
            );
}
