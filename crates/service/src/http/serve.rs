use std::net::SocketAddr;

use tower_http::ServiceBuilderExt;

use crate::http::helper::request_uuid::RequestUuid;

pub async fn serve(ip_addr: SocketAddr) {
    let trace_fro_http = tower_http::trace::TraceLayer::new_for_http()
        .make_span_with(tower_http::trace::DefaultMakeSpan::new().include_headers(true))
        .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(
            tower_http::trace::DefaultOnResponse::new()
                .level(tracing::Level::INFO)
                .latency_unit(tower_http::LatencyUnit::Micros), // on so on for `on_eos`, `on_body_chunk`, and `on_failure`
        );
    let middleware = tower::ServiceBuilder::new()
        .compression()
        .trim_trailing_slash()
        .set_x_request_id(RequestUuid)
        .layer(trace_fro_http)
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
