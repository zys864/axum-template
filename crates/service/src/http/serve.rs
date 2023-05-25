use std::net::SocketAddr;

use tower_http::ServiceBuilderExt;

use crate::http::helper::request_uuid::RequestUuid;

pub async fn serve(ip_addr: SocketAddr) {
    let middleware = tower::ServiceBuilder::new()
        .compression()
        .trim_trailing_slash()
        .set_x_request_id(RequestUuid)
        .trace_for_http()
        .propagate_x_request_id();
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
