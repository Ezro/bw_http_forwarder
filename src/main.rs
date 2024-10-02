use axum::{extract::Path, routing::get, Extension, Router};
mod bw_port_finder;

#[tokio::main]
async fn main() {
    // match bw_port_finder::get_port().await {
    //     Some(port) => {
    //         println!("Found working port: {}", port);
    //         let app = Router::new().route("/", get(root)).layer(Extension(port));
    //         let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //             .await
    //             .unwrap();
    //         axum::serve(listener, app).await.unwrap();
    //     }
    //     None => println!("No working port found."),
    // }
    match bw_port_finder::get_port().await {
        Some(port) => {
            println!("Found working port: {}", port);
            let app = Router::new()
                .route("/*path", get(forward_request)) // Catch all routes
                .layer(Extension(port));
            let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
                .await
                .unwrap();
            println!("Listening on http://127.0.0.1:3000");
            axum::serve(listener, app).await.unwrap();
        }
        None => println!("No working port found."),
    }
}

// async fn root(port: Extension<u16>, endpoint: str) -> String {
//     let url = format!("http://127.0.0.1:{}/{}", port.0, endpoint);
//     match reqwest::get(&url).await {
//         Ok(response) => {
//             if response.status().is_success() {
//                 // Return the response text
//                 return response
//                     .text()
//                     .await
//                     .unwrap_or_else(|_| "Failed to read response".to_string());
//             } else {
//                 return format!("Received non-success response: {}", response.status());
//             }
//         }
//         Err(err) => {
//             eprintln!("Error sending request: {}", err);
//             return "Error sending request".to_string();
//         }
//     }
// }

async fn forward_request(Extension(port): Extension<u16>, Path(path): Path<String>) -> String {
    let url = format!("http://127.0.0.1:{}/{}", port, path);
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                // Return the response text
                return response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Failed to read response".to_string());
            } else {
                return format!("Received non-success response: {}", response.status());
            }
        }
        Err(err) => {
            eprintln!("Error sending request: {}", err);
            return "Error sending request".to_string();
        }
    }
}
