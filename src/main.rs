use axum::{
    body::Body,
    http::{Request, Response},
    middleware::{self, Next},
    routing::get,
    Json, Router,
};
use serde_json::json;
use tower_http::services::ServeDir;

async fn status() -> Json<serde_json::Value> {
    Json(json!({
        "service": "Wii Web",
        "server": "online",
        "emulation_core": "Dolphin WebAssembly (experimental)",
        "core_revision": "7e38409ace3dda709c178312ff63fd92a3653cc7",
        "supported_formats": [".iso", ".wbfs", ".rvz", ".gcm"]
    }))
}

async fn cross_origin_isolation(request: Request<Body>, next: Next) -> Response<Body> {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("cross-origin-opener-policy", "same-origin".parse().unwrap());
    headers.insert(
        "cross-origin-embedder-policy",
        "require-corp".parse().unwrap(),
    );
    response
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/status", get(status))
        .fallback_service(ServeDir::new("core/wasm-dolphin").append_index_html_on_directories(true))
        .layer(middleware::from_fn(cross_origin_isolation));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to 127.0.0.1:3000");
    println!("Wii Web is ready at http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("server failed");
}
