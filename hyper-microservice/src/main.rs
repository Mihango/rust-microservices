#![allow(unused)]

use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use futures::{future, Future};
use hyper::service::{service_fn, service_fn_ok};

fn main() {
    let address = ([127, 0, 0, 1], 8080).into();
    hyper::rt::run(Server::bind(&address)
        .serve(|| { service_fn(microservice_handler) })
        .map_err(drop)
    );
}

const INDEX: &'static str = r#"
 <!doctype html>
 <html>
     <head>
         <title>Rust Microservice</title>
     </head>
     <body>
         <h3>Rust Microservice</h3>
     </body>
 </html>
 "#;


fn add_numbers(num1: u32, num2: Option<u32>) -> u32 {
    num1 + num2.unwrap_or(1)
}

fn microservice_handler(req: Request<Body>) -> impl Future<Item=Response<Body>, Error=Error> {

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            future::ok(Response::new(INDEX.into()))
        }
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();

            future::ok(response)
        }
    }
}