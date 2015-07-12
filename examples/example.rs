
extern crate ease;

use ease::Client;
use ease::UserAgent;

fn main() {
    let url = "https://api.discogs.com/releases/249504";
    println!("{}", Client::new(url)
                        .ok().expect("Couldn't parse URL.")
                        .header(UserAgent("FooBarApp/2.0".to_owned())).get().unwrap()
            );
}
