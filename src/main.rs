use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

// HTTPハンドラーの定義
async fn hello_world() -> &'static str {
    "Hello, World!"
}

// ヘルスチェック用のハンドラーの定義
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() -> Result<()> {
    // ルーターの定義
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/health", get(health_check));

    // localhost:8080 でリクエストを待ち受けるリスナーを作成
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    // サーバーを起動
    Ok(axum::serve(listener, app).await?)
}
