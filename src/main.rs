mod metrics;
mod storage;
mod web;

use metrics::collect_metrics;
use storage::{init_db, save_metrics};

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use axum::serve;

#[tokio::main]
async fn main() {
    // Wrap SQLite connection in Arc<Mutex<>>
    let conn: Arc<Mutex<Connection>> = Arc::new(Mutex::new(init_db()));

    // Background task to collect metrics
    let conn_clone = Arc::clone(&conn);
    tokio::spawn(async move {
        loop {
            let m = collect_metrics();
            {
                // Lock only inside this block, then drop before await
                let mut db = conn_clone.lock().unwrap();
                save_metrics(&mut db, &m);
            }
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    // Start web server
    let app = web::build_router();
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("LiteGauge running on http://{}", addr);

    serve(listener, app.into_make_service())
        .await
        .unwrap();
}
