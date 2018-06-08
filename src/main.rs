#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

#[get("/")]
fn index(req: RemoteAddr) -> String {
    req.addr()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket().launch();
}

struct RemoteAddr {
    addr: String,
}

impl RemoteAddr {
    fn addr(self) -> String {
        self.addr
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RemoteAddr {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
        if req.headers().contains("X-Forwarded-For") {
            let header = req.headers().get_one("X-Forwarded-For").unwrap();
            let addr = header.split(',').next().unwrap();

            return Outcome::Success(RemoteAddr {
                addr: addr.to_string(),
            });
        }

        Outcome::Success(RemoteAddr {
            addr: req.remote().unwrap().ip().to_string(),
        })
    }
}
