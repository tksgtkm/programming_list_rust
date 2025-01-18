use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

// tokioランタイム上で動かすために必要なマクロ
// このマクロを使用するとmain関数を非同期化できる
#[tokio::main]
async fn main() -> Result<()> {
    // ルーターと呼ばれるものを設定する
    // 今回は「/hello」というパスに対してGETリクエストが来たら
    // hello_world関数を呼び出すように設定している
    let app = Router::new().route("/health", get(health_check));

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}
