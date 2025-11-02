use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

// HTTPハンドラーの定義
async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // ルーターの定義
    let app = Router::new().route("/hello", get(hello_world));

    // localhost:8080 でリクエストを待ち受けるリスナーを作成
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    // サーバーを起動
    axum::serve(listener, app).await.unwrap();
}
