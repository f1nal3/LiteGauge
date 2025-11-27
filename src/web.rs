use axum::{Router, routing::get, Json};
use crate::metrics::collect_metrics;

async fn metrics_endpoint() -> Json<crate::metrics::Metrics> {
    let m = collect_metrics();
    Json(m)
}

pub fn build_router() -> Router {
    Router::new()
        .route("/metrics", get(metrics_endpoint))
        .route("/", get(|| async { "LiteGauge: Container Metrics Agent" }))
}
