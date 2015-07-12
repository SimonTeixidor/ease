
extern crate ease;

use ease::Client;

fn main() {
    println!("{}", 
             Client::new("http://httpbin.org/get")
                        .ok().expect("Couldn't parse URL.")
                        .get()
                        .unwrap()
            );

    println!("\n");

    println!("{}", 
             Client::new("http://httpbin.org/post")
                        .ok().expect("Couldn't parse URL.")
                        .param(("foo", "bar"))
                        .post()
                        .unwrap()
            );

    println!("{}", 
             Client::new("http://httpbin.org/put")
                        .ok().expect("Couldn't parse URL.")
                        .param(("foo", "bar"))
                        .put()
                        .unwrap()
            );

    println!("{}", 
             Client::new("http://httpbin.org/delete")
                        .ok().expect("Couldn't parse URL.")
                        .param(("foo", "bar"))
                        .delete()
                        .unwrap()
            );
}
