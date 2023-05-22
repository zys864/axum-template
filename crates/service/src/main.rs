use std::{future::Future, sync::atomic::AtomicUsize};

pub mod core;
pub mod error;
pub mod http;
pub mod model;
pub mod utils;

pub type DbPool = sqlx::PgPool;
fn main() {
    dotenvy::dotenv().ok();
    let _guard = utils::tracing_utils::tracing_init();
    tracing::info!("starting ...");
    let ip_addr = "0.0.0.0:3000".parse().unwrap();
    block_on(http::serve::serve(ip_addr))
}

fn runtime() -> std::io::Result<tokio::runtime::Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            format!("pool-{}", id)
        })
        .enable_all()
        .build()
}
fn block_on<F: Future>(future: F) -> F::Output {
    let rt = runtime().expect("start tokio runtime failed");
    rt.block_on(future)
}
