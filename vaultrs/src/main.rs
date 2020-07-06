extern crate base64;

use dotenv;
use futures::prelude::*;
use hashicorp_vault::client::VaultClient as Client;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use regex::Regex;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

/// TODO: Move this to dotenv
const VAULT_HOST: &str = "http://127.0.0.1:8200";
const VAULT_TOKEN: &str = "s.tLB4B0dfq7j9tlVCqdkoZtrE";

// Routes Regex
lazy_static! {
    static ref ENCRYPT_PATH: Regex = Regex::new(r"^/encrypt/(?P<payload>.*)").unwrap();
    static ref DECRYPT_PATH: Regex = Regex::new(r"^/decrypt/(?P<payload>.*)").unwrap();
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn requests_handler(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, path) if ENCRYPT_PATH.is_match(path) => {
            let encrypt_str = ENCRYPT_PATH
                .captures(path)
                .unwrap()
                .name("payload")
                .unwrap()
                .as_str();

            // TODO: Move this to a function and pass the client as a reference
            let client = Client::new(VAULT_HOST, VAULT_TOKEN).unwrap();
            let key_id = "farm";
            let enc_resp = client.transit_encrypt(None, key_id, encrypt_str).unwrap();

            *response.body_mut() = Body::from(format!("Encrypted: {}", base64::encode(enc_resp)));
        }
        (&Method::GET, path) if DECRYPT_PATH.is_match(path) => {
            let decrypt_str = DECRYPT_PATH
                .captures(path)
                .unwrap()
                .name("payload")
                .unwrap()
                .as_str();

            // TODO: Move this to a function and pass the client as a reference
            // TODO: Pass the key_id in the URL
            let key_id = "farm";
            let decoded = base64::decode(decrypt_str).unwrap();

            let client = Client::new(VAULT_HOST, VAULT_TOKEN).unwrap();
            let dec_resp = client.transit_decrypt(None, key_id, decoded);
            let payload = dec_resp.unwrap();

            *response.body_mut() = Body::from(format!("Decrypted: {}", base64::encode(payload)));
        }
        _ => {
            *response.status_mut() = StatusCode::METHOD_NOT_ALLOWED;
        }
    }

    future::ok(response).await
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv::dotenv().expect("Failed to read .env file");

    let addr: SocketAddr = env::var("SERVICE_ADDRESS")
        .unwrap_or_else(|_| "127.0.0.1:8080".into())
        .parse()
        .expect("can't parse SERVICE_ADDRESS variable");

    // Service to handle connections
    let service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(requests_handler)) });

    // Bind service to the socket and serve
    let server = Server::bind(&addr).serve(service);

    // Graceful shutdown signal
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // we capture all errors
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hashicorp_vault::client::VaultClient as Client;

    #[test]
    fn it_can_create_a_client() {
        let _ = Client::new(HOST, TOKEN).unwrap();
    }

    #[test]
    fn it_can_transit_encryption() {
        let client = Client::new(HOST, TOKEN).unwrap();
        let res = client.transit_encrypt(None, "keyname", b"plaintext");
        assert!(res.is_ok());
    }

    #[test]
    fn it_can_encrypt_decrypt_transit() {
        let key_id = "test-vault-rs";
        let plaintext = b"data\0to\0encrypt";

        let client = Client::new(HOST, TOKEN).unwrap();
        let enc_resp = client.transit_encrypt(None, key_id, plaintext);
        let encrypted = enc_resp.unwrap();
        let dec_resp = client.transit_decrypt(None, key_id, encrypted);
        let payload = dec_resp.unwrap();
        assert_eq!(plaintext, payload.as_slice());
    }

    #[test]
    fn it_list_secrets() {
        let client = Client::new(HOST, TOKEN).unwrap();
        let res = client.set_secret("hello/fred", "world");
        assert!(res.is_ok());
        let res = client.set_secret("hello/bob", "world");
        assert!(res.is_ok());
        let res = client.list_secrets("hello/");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), ["bob", "fred"]);
    }
}
