#![allow(unused)]

use std::fmt;
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};
use std::thread::park_timeout;

use futures::{future, Future};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use hyper::service::{service_fn, service_fn_ok};
use lazy_static::lazy_static;
use regex::Regex;
use slab::Slab;

fn main() {
    let address = ([127, 0, 0, 1], 8080).into();
    let user_db = Arc::new(Mutex::new(Slab::new()));
    hyper::rt::run(Server::bind(&address)
        .serve(move || {
            let user_db = user_db.clone();
            service_fn(move |req| microservice_handler_regex(req, &user_db))
        })
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

const USER_PATH_1: &str = "/user/";

lazy_static! {
    static ref INDEX_PATH: Regex = Regex::new("^/(index\\.html?)?$").unwrap();
    static ref USER_PATH: Regex = Regex::new("^/user/((?P<user_id>\\d+?)/?)?$").unwrap();
    static ref USERS_PATH: Regex = Regex::new("^/users/?$").unwrap();
}

fn add_numbers(num1: u32, num2: Option<u32>) -> u32 {
    num1 + num2.unwrap_or(1)
}

fn response_with_code(status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap()
}

fn microservice_handler_regex(req: Request<Body>, user_db: &UserDb) -> impl Future<Item=Response<Body>, Error=Error> {
    let response = {
        let method = req.method();
        let path = req.uri().path();
        let mut users = user_db.lock().unwrap();

        if INDEX_PATH.is_match(path) {
            if method == &Method::GET {
                Response::new(INDEX.into())
            } else {
                response_with_code(StatusCode::METHOD_NOT_ALLOWED)
            }
        } else if USERS_PATH.is_match(path) {
            if method == &Method::GET {
                let list = users.iter()
                    .map(|(id, _)| id.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                Response::new(list.into())
            } else {
                response_with_code(StatusCode::METHOD_NOT_ALLOWED)
            }
        } else if let Some(cap) = USER_PATH.captures(path) {
            let user_id = cap.name("user_id").and_then(|m| {
                m.as_str()
                    .parse::<UserId>()
                    .ok()
                    .map(|x| x as usize)
            });

            match (method, user_id) {
                (&Method::POST, None) => {
                    let id = users.insert(UserData);
                    Response::new(id.to_string().into())
                }
                (&Method::POST, Some(_)) => {
                    response_with_code(StatusCode::BAD_REQUEST)
                }
                (&Method::GET, Some(id)) => {
                    if let Some(data) = users.get(id) {
                        Response::new(data.to_string().into())
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                }
                (&Method::PUT, Some(id)) => {
                    if let Some(user) = users.get_mut(id) {
                        *user = UserData;
                        response_with_code(StatusCode::OK)
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                }
                (&Method::DELETE, Some(id)) => {
                    if users.contains(id) {
                        users.remove(id);
                        response_with_code(StatusCode::OK)
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                }
                _ => {
                    response_with_code(StatusCode::METHOD_NOT_ALLOWED)
                }
            }
        } else {
            response_with_code(StatusCode::NOT_FOUND)
        }
    };

    future::ok(response)
}

// fn microservice_handler(req: Request<Body>, user_db: &UserDb) -> impl Future<Item=Response<Body>, Error=Error> {
//     let response = match (req.method(), req.uri().path()) {
//         (&Method::GET, "/") => {
//             Response::new(INDEX.into())
//         }
//         (method, path) if path.starts_with(USER_PATH_1) => {
//             let user_id = path.trim_left_matches(USER_PATH_1)
//                 .parse::<UserId>()
//                 .ok()
//                 .map(|x| x as usize);
//             let mut users = user_db.lock().unwrap();
//             match (method, user_id) {
//                 (&Method::POST, None) => {
//                     let id = users.insert(UserData);
//                     Response::new(id.to_string().into())
//                 }
//                 (&Method::POST, Some(_)) => {
//                     response_with_code(StatusCode::BAD_REQUEST)
//                 }
//                 (&Method::GET, Some(id)) => {
//                     if let Some(data) = users.get(id) {
//                         Response::new(data.to_string().into())
//                     } else {
//                         response_with_code(StatusCode::NOT_FOUND)
//                     }
//                 }
//                 (&Method::PUT, Some(id)) => {
//                     if let Some(user) = users.get_mut(id) {
//                         *user = UserData;
//                         response_with_code(StatusCode::OK)
//                     } else {
//                         response_with_code(StatusCode::NOT_FOUND)
//                     }
//                 }
//                 (&Method::DELETE, Some(id)) => {
//                     if users.contains(id) {
//                         users.remove(id);
//                         response_with_code(StatusCode::OK)
//                     } else {
//                         response_with_code(StatusCode::NOT_FOUND)
//                     }
//                 }
//                 _ => {
//                     response_with_code(StatusCode::METHOD_NOT_ALLOWED)
//                 }
//             }
//         }
//         _ => {
//             response_with_code(StatusCode::NOT_FOUND)
//         }
//     };
//     future::ok(response)
// }

type UserId = u64;

struct UserData;

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("{}")
    }
}

type UserDb = Arc<Mutex<Slab<UserData>>>;