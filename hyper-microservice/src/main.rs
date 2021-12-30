#![allow(unused)]
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 8082).into();
    let builder = Server::bind(&addr);
    let new_service = make_service_fn(move |_| async {
        Ok::<_, hyper::Error>(service_fn(|req| handle_services(req)))
    });
    let server = builder.serve(new_service);
    if let Err(e) = server.await {
        eprint!("Server error {}", e);
    };
    Ok(())
}

async fn handle_services(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Response::builder()
                .header("Content-Type", "application/json")
                .body("{\"msg\": \"Hello hyper crate\"}".into())
                .unwrap()
        },
        (method, path) if path.starts_with("/user") => {
            response_with_body(StatusCode::OK, Body::from("{\"data\": \"user endpoint\"}"))
        },
        (method, path) if path.starts_with("/articles") => {
            response_with_body(StatusCode::OK, Body::from("{\"data\": \"articles endpoint\"}"))
        },
        (method, path) if path.starts_with("/comments") => {
            response_with_body(StatusCode::OK, Body::from("{\"data\": \"comments endpoint\"}"))
        },
        _ => { response_with_code(StatusCode::NOT_FOUND)}
    };
    Ok(response)
}

fn response_with_code(status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .unwrap()
}

fn response_with_body(status: StatusCode, body: Body) -> Response<Body> {
    Response::builder()
        .status(status)
        .body(body)
        .unwrap()
}
