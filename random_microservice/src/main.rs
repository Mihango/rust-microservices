use std::env;
use std::net::SocketAddr;
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use log::{debug, info, log_enabled, trace, warn};
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use dotenv::dotenv;
use serde_derive::Deserialize;
use std::io::{self, Read};
use std::fs::File;

/// endable logging on cargo run command - RUST_LOG=random_microservice=trace,warn  RUST_LOG_STYLE=auto
static OWN_LOG_VAR: &str = "LOG_LEVEL";
static OWN_LOG_STYLE_VAR: &str = "LOG_STYLE";

fn main() {
    // read variables from enviroment -- old way or reading variables
    let log_style = env::var("LOG_STYLE").unwrap_or_else(|_| String::from("auto"));

    let log_level =
        env::var("LOG_LEVEL").unwrap_or_else(|_| String::from("random_microservice=trace"));

    // println!("Log level >>>> {} and style >>>> {}", log_level, log_style);

    // setting own rust log and style from env
    let env = env_logger::Env::new()
        .filter(log_level)
        .write_style(log_style);

    // let addr = env::var("ADDRESS")
    //     .unwrap_or_else(|_| "127.0.0.1".into())
    //     .parse()
    //     .expect("Can't parse ADDRESS variable");

    // reading variables using dotenv and parsing with clap crate
    let matches = create_parser();
    let addr: SocketAddr = matches
        .value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("ADDRESS").ok())
        .unwrap_or_else(|| "127.0.0.1:8080".into())
        .parse()
        .expect("Can't parse ADDRESS variable");

    env_logger::init_from_env(env);

    info!("Rand Microservice - v0.1.0");
    trace!("Starting...");

    // let addr: SocketAddr = ([127, 0, 0, 1], 8001).into();

    if log_enabled!(log::Level::Debug) {
        debug!("Trying to bind server to address {}", addr);
    }

    let builder = Server::bind(&addr);

    trace!("Creating service handler...");

    let server = builder.serve(|| {
        service_fn_ok(|_| {
            let random_byte = rand::random::<u8>();
            Response::new(Body::from(random_byte.to_string()))
        })
    });

    info!("Used address: {}", server.local_addr());
    let server = server.map_err(drop);
    debug!("Run!");
    hyper::rt::run(server);
}

fn create_parser() -> ArgMatches {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("ADDRESS")
                .about("Sets an address")
                .takes_value(true),
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();
    matches
}

#[derive(Deserialize)]
struct Config {
    address: SocketAddr,
}

fn read_config_file() {
    let config = File::open("microservices.toml")
    .and_then(|mut file| {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer);
        Ok(buffer)
    })
    .and_then(|buffer| {
        toml::from_str::<Config>(&buffer)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
    })
    .map_err(|err| {
        warn!("Can't read config file {}", err);
    })
    .ok();
}
