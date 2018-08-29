extern crate actix_web;

use actix_web::{server, App, HttpRequest};

fn index(_req: &HttpRequest) -> &'static str {
    "Hello, world!"
}

fn home(_req: &HttpRequest) -> &'static str {
    "Welcome Home!"
}

fn main() {
    let a_server = server::new(||{
        vec![ 
            App::new()
                .prefix("/api")
                .resource("/", |r| r.f(index)),
            App::new()
                .resource("/", |r| r.f(home))
        ]
    });
    
    a_server.bind("127.0.0.1:8088")
        .unwrap()
        .run();
}