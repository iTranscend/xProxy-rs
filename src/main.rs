use anyhow::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Error, Request, Server};
use std::net::SocketAddr;
use std::sync::Arc;

fn mutate_request(req: &mut Request<Body>) -> Result<()> {
    // Stripping off unnecessary headers
    req.headers_mut().remove("content-length");
    req.headers_mut().remove("transfer-encoding");
    req.headers_mut().remove("accept-encoding");
    req.headers_mut().remove("content-encoding");

    // A uri is split into:
    // Scheme(e.g. https),
    // authority(e.g. www.pillowdesk.com or 192.128.0.1:4300),
    // path(e.g /helpdesk/withdrawals),
    // query(e.g. ?status=withdrawn) and
    // fragments(e.g. #fragid)

    // format uri with respect to whether a query is present  or not
    let uri = req.uri();
    let uri_string = match uri.query() {
        None => format!("https://www.upwork.com{}", uri.path()),
        Some(query) => format!("https://www.upwork.com{}{}", uri.path(), query),
    };
    println!("URI: {}", &uri_string);
    *req.uri_mut() = uri_string
        .parse()
        .context("parsing the uri in mutate_request")?;
    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: Client<_, hyper::Body> = Client::builder().build(https);
    let client: Arc<Client<_, hyper::Body>> = Arc::new(client);

    // Server-side listener
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Make service to handle each connection
    let make_svc = make_service_fn(move |_| {
        let client = Arc::clone(&client);
        async move {
            Ok::<_, Error>(service_fn(move |mut req| {
                let client = Arc::clone(&client);
                async move {
                    mutate_request(&mut req)?;
                    client
                        .request(req)
                        .await
                        .context("Sending request to backend server")
                }
            }))
        }
    });
    Server::bind(&addr)
        .serve(make_svc)
        .await
        .context("500: Internal Server error")?;

    Ok::<(), anyhow::Error>(())
}
