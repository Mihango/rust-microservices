#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate base64;
#[macro_use]
extern crate base64_serde;

mod color;

use base64::STANDARD;
use color::Color;
use futures::{future, Future, Stream};
use hyper::service::service_fn;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use rand::distributions::{Bernoulli, Normal, Uniform};
use rand::Rng;
use std::cmp::{max, min};
use std::ops::Range;

fn main() {
    let add = ([127, 0, 0, 1], 8000).into();
    let builder = Server::bind(&add);
    let server = builder.serve(|| service_fn(microservice_handler));
    let server = server.map_err(drop);
    hyper::rt::run(server);
}

#[derive(Serialize)]
struct RngResponse {
    value: f64,
}

/// deserialization will be as follows { "Uniform": { "range": { "start": 1, "end": 10 } } }
#[derive(Deserialize)]
enum RngRequest {
    Uniform {
        range: Range<i32>,
    },
    Normal {
        mean: f64,
        #[serde(rename = "stdDev")]
        std_dev: f64,
    },
    Bernoulli {
        p: f64,
    },
    Shuffle {
        #[serde(with = "Base64Standard")]
        data: Vec<u8>,
    },
    Color {
        from: Color,
        to: Color,
    },
}

base64_serde_type!(Base64Standard, base64::STANDARD);

/// if you want to use { "distribution": "uniform", "parameters": { "start": 1, "end": 10 } } }
/// you can declare the request as follows
#[derive(Deserialize)]
#[serde(tag = "distribution", content = "parameters", rename_all = "camelCase")]
enum RngRequestEx {
    Uniform {
        #[serde(flatten)]
        range: Range<i32>,
    },
    Normal {
        mean: f64,
        std_dev: f64,
    },
    Bernoulli {
        p: f64,
    },
}

/// sample to flattern -- Remote Procedure Call
/// the params field contains an array of any json values
/// if you serialize an instance of this struct it will include the variant 1.e
/// { "Request": { "id": 1, "method": "get_user", "params": [123] } }
/// To remove this behavour use #[serde(untagged)]  - result: { "id": 1, "method": "get_user", "params": [123] }
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum RpcRequest {
    Request {
        id: u32,
        method: String,
        params: Vec<Value>,
    },
    Notification {
        id: u32,
        method: String,
        params: Vec<Value>,
    },
}

fn microservice_handler(
    req: Request<Body>,
) -> Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/random") => {
            let body = req.into_body().concat2().map(move |chunks: Chunk| {
                let res = serde_json::from_slice::<RngRequest>(chunks.as_ref())
                    .map(handle_request)
                    .and_then(|resp| serde_json::to_string(&resp));

                match res {
                    Ok(body) => Response::new(body.into()),
                    Err(err) => Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(err.to_string().into())
                        .unwrap(),
                }
            });
            Box::new(body)
        }
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Not Found".into())
                .unwrap();
            Box::new(future::ok(response))
        }
    }
}

fn handle_request(req: RngRequest) -> RngResponse {
    let mut rng = rand::thread_rng();
    let value = {
        match req {
            RngRequest::Uniform { range } => rng.sample(Uniform::from(range)) as f64,
            RngRequest::Normal { mean, std_dev } => rng.sample(Normal::new(mean, std_dev)) as f64,
            RngRequest::Bernoulli { p } => rng.sample(Bernoulli::new(p)) as i8 as f64,
        }
    };
    RngResponse { value }
}
