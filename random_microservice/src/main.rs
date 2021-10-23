use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn main() {
    let address = ([127, 0, 0, 1], 8001).into();
    let builder = Server::bind(&address);
    let server = builder.serve(|| {
        service_fn_ok(|_| {
            let random_byte = rand::random::<u8>();
            Response::new(Body::from(random_byte.to_string()))
        })
    });
    let server = server.map_err(drop);
    hyper::rt::run(server)
}