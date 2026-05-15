mod data;
mod engine;
mod models;
mod render;

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use models::PackageCollection;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(5048);

    let app = app();
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = match tokio::net::TcpListener::bind(address).await {
        Ok(listener) => listener,
        Err(_) => {
            eprintln!(
                "Dependency Drift Watch could not start because port {} is already in use.",
                port
            );
            eprintln!("Set a different port before running again, for example:");
            eprintln!("$env:PORT = \"5052\"");
            eprintln!("cargo run");
            std::process::exit(1);
        }
    };

    println!("Dependency Drift Watch listening on http://127.0.0.1:{port}/");
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/docs", get(docs))
        .route("/packages", get(packages_page))
        .route("/review-queue", get(review_queue_page))
        .route("/owners", get(owners_page))
        .route("/metrics-preview", get(metrics_preview))
        .route("/api/dashboard/summary", get(summary))
        .route("/api/packages", get(packages))
        .route("/api/packages/{id}", get(package))
        .route("/api/review-queue", get(review_queue))
        .route("/api/owners", get(owners))
        .route("/api/policy", get(policy))
        .route("/api/audit", get(audit_log))
        .route("/api/sample", get(sample))
        .route("/metrics", get(metrics))
}

async fn root() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(render::render_overview()),
    )
}

async fn docs() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(render::render_docs()),
    )
}

async fn packages_page() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(render::render_packages()),
    )
}

async fn review_queue_page() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(render::render_review_queue()),
    )
}

async fn owners_page() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(render::render_owners()),
    )
}

async fn metrics_preview() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(render::render_metrics_preview()),
    )
}

async fn summary() -> Json<models::DashboardSummary> {
    Json(engine::dashboard_summary())
}

async fn packages() -> Json<PackageCollection> {
    Json(engine::package_collection())
}

async fn package(Path(id): Path<String>) -> impl IntoResponse {
    match engine::package(&id) {
        Some(pkg) => (StatusCode::OK, Json(pkg)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "package_not_found",
                "id": id
            })),
        )
            .into_response(),
    }
}

async fn review_queue() -> Json<Vec<models::PackageAssessment>> {
    Json(engine::review_queue())
}

async fn owners() -> Json<Vec<models::OwnerLane>> {
    Json(engine::owner_lanes())
}

async fn policy() -> Json<models::PolicyConfig> {
    Json(engine::policy_config())
}

async fn audit_log() -> Json<Vec<models::AuditEvent>> {
    Json(engine::audit_log())
}

async fn sample() -> Json<models::SamplePayload> {
    Json(engine::sample_payload())
}

async fn metrics() -> impl IntoResponse {
    (
        [(CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")],
        engine::prometheus_metrics(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn root_route_returns_ok() {
        let response = app()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn missing_package_returns_404() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/api/packages/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn metrics_route_returns_ok() {
        let response = app()
            .oneshot(Request::builder().uri("/metrics").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
