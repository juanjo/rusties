use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello Vault")))
}

#[tokio::main]
async fn main() {
    // Socket to listen to
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Service to handle connections
    let service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Bind service to the socket and serve
    let server = Server::bind(&addr).serve(service);

    // we capture all errors
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
