use futures::io::BufWriter;
use futures::{StreamExt, TryFutureExt};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};
use std::{fs, path::Path};
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let files = Path::new("./files");
    fs::create_dir(files).ok();

    let addr = ([127, 0, 0, 1], 8080).into();

    let make_svc = make_service_fn(|_| async {
        Ok::<_, Error>(service_fn(|req| async { microservice_handler(req, files) }))
    });
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);
    let _ = server.await;
}

fn microservice_handler(req: Request<Body>, files: &Path) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path().to_owned().as_ref()) {
        (&Method::GET, "/") => Ok(Response::new("Hello".into())),
        (&Method::POST, "/upload") => {
            let name: String = thread_rng().sample_iter(&Alphanumeric).take(20).collect();
            let mut filepath = files.to_path_buf();
            filepath.push(&name);

            let create_file = File::create(filepath);
            let write = create_file.and_then(|file| {
                req.into_body().fold(file, |file, chunk| {
                    let mut writer = BufWriter::new(file);

                    // Write a byte to the buffer.
                    writer.write(&[42u8]).await?;

                    // Flush the buffer before it goes out of scope.
                    writer.flush().await?;
                })
            });

            Ok(Response::new("Hello".into()))
        }
        _ => response_with_code(StatusCode::NOT_FOUND),
    }
}

fn response_with_code(status_code: StatusCode) -> Result<Response<Body>, hyper::Error> {
    let resp = Response::builder()
        .status(status_code)
        .body(Body::empty())
        .unwrap();
    Ok(resp)
}
