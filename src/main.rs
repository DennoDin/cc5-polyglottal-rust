// extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use actix_web::*;
// use bytes::Bytes;
// use futures::stream::once;
// use futures::future::{Future, result};

use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http::*};

#[derive(Serialize)]
struct MyObj {
    name: &'static str
}

impl Responder for MyObj {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn index(_req: &HttpRequest) -> impl Responder {
    MyObj{
        name: "Dustin"
    }
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
                .resource("/", |r| {
                    r
                        .route("", Method::GET, home)
                        .route("/index", Method::GET, index)
                })
                //r.f(home))
                // .resource("/index", |r| r.f(index))
        ]
    });
    
    a_server.bind("127.0.0.1:8088")
        .unwrap()
        .run();
}