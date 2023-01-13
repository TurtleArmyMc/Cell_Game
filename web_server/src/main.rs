use hyper::http::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{header, Body, Method, Request, Response, Server, StatusCode};
use std::{convert::Infallible, net::SocketAddr};
use tokio::io::AsyncReadExt;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    println!("{} REQUEST:\t{}", req.method(), req.uri().path());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            static_file(
                &mut response,
                "www/static/index.html",
                HeaderValue::from_static("text/html"),
            )
            .await
        }
        (&Method::GET, "/cell_game.js") => {
            static_file(
                &mut response,
                "www/static/cell_game/cell_game.js",
                HeaderValue::from_static("text/javascript"),
            )
            .await
        }
        (&Method::GET, "/cell_game_bg.wasm") => {
            static_file(
                &mut response,
                "www/static/cell_game/cell_game_bg.wasm",
                HeaderValue::from_static("application/wasm"),
            )
            .await
        }
        (method, path) => {
            println!("{} REQUEST 404:\t{}", method, path);
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };
    Ok(response)
}

async fn static_file(response: &mut Response<Body>, file_path: &str, content_type: HeaderValue) {
    let mut s = Vec::new();
    match tokio::fs::File::open(file_path).await {
        Ok(mut file) => match file.read_to_end(&mut s).await {
            Ok(_) => {
                *response.body_mut() = Body::from(s);
                response
                    .headers_mut()
                    .insert(header::CONTENT_TYPE, content_type);
            }
            Err(e) => eprintln!("{}", e),
        },
        Err(e) => eprintln!("{}", e),
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
