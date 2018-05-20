extern crate iron;
#[macro_use]
extern crate hyper;
extern crate logger;

use iron::method;
use iron::prelude::*;
use iron::status;
use logger::format::Format;
use logger::Logger;
use std::net::IpAddr;
use std::str::FromStr;

header! { (XForwardedFor, "X-Forwarded-For") => [String] }

fn main() {
    let format = Format::new(
        "@[bold]{method}@ {uri} @[bold]->@ @[C]{ip-addr}@ -- @[C]{status}@ \
         ({response-time})",
        vec![],
        vec![],
    );

    let (logger_before, logger_after) = Logger::new(Some(format.unwrap()));

    let mut chain = Chain::new(root);

    // Link logger_before as your first before middleware.
    chain.link_before(logger_before);

    // Link logger_after as your *last* after middleware.
    chain.link_after(logger_after);

    let binding = "0.0.0.0:8080";
    Iron::new(chain).http(binding).unwrap();
}

fn remote_or_xforwarded_for(req: &Request) -> IpAddr {
    if req.headers.has::<XForwardedFor>() {
        let ip_raw = req.headers.get::<XForwardedFor>().unwrap().as_str();
        return IpAddr::from_str(ip_raw).unwrap();
    }

    req.remote_addr.ip()
}

fn root(req: &mut Request) -> IronResult<Response> {
    Ok(match req.method {
        method::Get => {
            let ip = remote_or_xforwarded_for(req);
            req.remote_addr.set_ip(ip);
            Response::with((status::Ok, format!("{}", ip)))
        }
        _ => Response::with(status::BadRequest),
    })
}
