use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use reqwest::header::CONTENT_TYPE;
mod bw_port_finder;

#[tokio::main]
async fn main() {
    match bw_port_finder::get_port().await {
        Some(port) => {
            println!("Found working port: {}", port);
            let app = Router::new()
                .route("/*path", get(forward_request))
                .layer(Extension(port));
            let addr = "0.0.0.0:3000";
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            println!("Listening on {:?}", addr);
            axum::serve(listener, app).await.unwrap();
        }
        None => println!("No working port found."),
    }
}

async fn forward_request(Extension(port): Extension<u16>, Path(path): Path<String>) -> Response {
    let url = format!("http://127.0.0.1:{}/{}", port, path);
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                let response_str = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to read response".to_string());
                let mut response = response_str.into_response();
                let headers = response.headers_mut();
                headers.clear();
                headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
                return response;
            } else {
                return format!("Received non-success response: {}", response.status())
                    .into_response();
            }
        }
        Err(err) => {
            eprintln!("Error sending request: {}", err);
            return "Error sending request".to_string().into_response();
        }
    }
}
