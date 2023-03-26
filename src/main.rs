#![feature(exclusive_range_pattern)]
#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::process::Command;

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use tokio::net::TcpListener;

async fn handle_request(req: Request<IncomingBody>) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/info") => get_network_info().await,
        (&hyper::Method::GET, "/halt") => halt().await,
        (&hyper::Method::GET, "/reboot") => reboot().await,
        _ => Ok(not_found()),
    }
}

async fn get_network_info() -> Result<Response<Full<Bytes>>, Infallible> {
    let output = Command::new("ip")
        .arg("addr")
        .output()
        .expect("Failed to execute ip addr command");

    let body = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(Response::new(Full::new(Bytes::from(body))))
}

async fn halt() -> Result<Response<Full<Bytes>>, Infallible> {
    Command::new("sudo")
        .arg("halt")
        .spawn()
        .expect("Failed to execute halt command");

    Ok(Response::new(Full::new(Bytes::from("System halt initiated."))))
}

async fn reboot() -> Result<Response<Full<Bytes>>, Infallible> {
    Command::new("sudo")
        .arg("reboot")
        .spawn()
        .expect("Failed to execute reboot command");

    Ok(Response::new(Full::new(Bytes::from("System reboot initiated."))))
}

fn not_found() -> Response<Full<Bytes>> {
    Response::builder()
        .status(hyper::StatusCode::NOT_FOUND)
        .body(Full::new(Bytes::from("Not Found")))
        .unwrap()
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = ([0, 0, 0, 0], 65535).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(handle_request))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
