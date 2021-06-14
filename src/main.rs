use anyhow::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Response, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    // Making a http request

    // Server-side listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server_handle = tokio::spawn(async move {
        // Make service to handle each connection
        let make_svc = make_service_fn(|_| async {
            Ok::<_, Error>(service_fn(|_req| async {
                Ok::<_, Error>(Response::new(Body::from("Hello World")))
            }))
        });
        Server::bind(&addr)
            .serve(make_svc)
            .await
            .context("500: Internal Server error")?;
        Ok::<(), anyhow::Error>(())
    });

    // Client-side get request
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: Client<_, hyper::Body> = Client::builder().build(https);

    let url = "http://127.0.0.1:3000".parse().context("Parsing URL")?;
    let res = client.get(url).await.context("Performing HTTP request")?;
    println!("{:?}", res);

    server_handle.await??;
    Ok(())
}
