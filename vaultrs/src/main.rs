use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use regex::Regex;
use std::convert::Infallible;
use std::net::SocketAddr;

// Routes Regex
lazy_static! {
    static ref ENCRYPT_PATH: Regex = Regex::new("^/encrypt/.*").unwrap();
    static ref DECRYPT_PATH: Regex = Regex::new("^/decrypt/.*").unwrap();
}

async fn requests_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, path) if ENCRYPT_PATH.is_match(path) => {
            *response.body_mut() = Body::from(format!("Encrypted: {}", path));
        }
        (&Method::GET, path) if DECRYPT_PATH.is_match(path) => {
            *response.body_mut() = Body::from(format!("Decrypted: {}", path));
        }
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        }
    }

    Ok(response)
}

#[tokio::main]
async fn main() {
    // Socket to listen to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Service to handle connections
    let service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(requests_handler)) });

    // Bind service to the socket and serve
    let server = Server::bind(&addr).serve(service);

    // we capture all errors
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
