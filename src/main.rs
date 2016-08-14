extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(r: &mut Request) -> IronResult<Response> {
        println!("Remote addr is {}", r.remote_addr.ip());
        Ok(Response::with((status::Ok, format!("{}", r.remote_addr.ip()))))
    }

    println!("On 3000");
    Iron::new(hello_world).http("localhost:3000").unwrap();
}
