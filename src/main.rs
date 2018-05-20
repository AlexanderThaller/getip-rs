#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request};

#[get("/")]
fn index<'a>(req: RemoteAddr) -> String {
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
            Outcome::Success(RemoteAddr {
                addr: format!("{}", req.headers().get_one("X-Forwarded-For").unwrap()),
            })
        } else {
            Outcome::Success(RemoteAddr {
                addr: format!("{}", req.remote().unwrap()),
            })
        }
    }
}
