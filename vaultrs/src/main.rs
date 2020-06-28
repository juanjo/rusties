use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn requests_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/encrypt") => {
            *response.body_mut() = Body::from("Encrypted");
        }
        (&Method::GET, "/decrypt") => {
            *response.body_mut() = Body::from("Decrypted");
        }
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        }
    };

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
