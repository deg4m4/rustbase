use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        let counter = 0;
        let cs = Arc::new(Mutex::new(counter));
        Ok::<_, Infallible>(service_fn(move |r| hello_world(r, cs.clone())))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(_req: Request<Body>, c: Arc<Mutex<i32>>) -> Result<Response<Body>, Infallible> {
    let body = hyper::body::to_bytes(_req.into_body()).await;
    println!("{:?}", body);
    Ok(Response::new(format!("Hello, {}!", "p").into()))
}
