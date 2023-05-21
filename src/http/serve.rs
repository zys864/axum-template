use std::net::SocketAddr;

use tower_http::ServiceBuilderExt;

pub async fn serve(ip_addr: SocketAddr) {
    let middleware = tower::ServiceBuilder::new()
        .compression()
        .trim_trailing_slash()
        .propagate_x_request_id()
        .trace_for_http();
    let app = crate::http::api_router().await.layer(middleware);
    tracing::info!("listening host: {}", ip_addr);
    axum::Server::bind(&ip_addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(crate::utils::graceful_shutdown::shutdown_signal(
            "http_server",
        ))
        .await
        .unwrap();
}
