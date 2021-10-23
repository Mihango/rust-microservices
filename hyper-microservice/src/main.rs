#![allow(unused)]

use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;

fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();
    println!("Address >>>> yet to convert {:?}", addr);
    let builder = Server::bind(&addr);
    let server = builder.serve(|| {
        service_fn(|_| Response::new(Body::from("Almost microservice -- testing watch")))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

fn add_numbers(num1: u32, num2: Option<u32>) -> u32 {
    num1 + num2.unwrap_or(1)
}

fn microservice_handler(req: Request<Body>) -> impl Future<Item = Response<Body>, Error = Error> {
    unimplemented!()
}
