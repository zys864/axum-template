use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_session: sqlx::PgPool,
    pub data: Arc<Mutex<HashMap<String, Box<dyn std::any::Any + Send>>>>,
}

impl AppState {
    pub async fn new() -> Self {
        let db_session = db_session().await;
        let data = Arc::new(Mutex::new(HashMap::with_capacity(256)));
        AppState { db_session, data }
    }
}

async fn db_session() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env not be setted");

    let conn = database_url
        .parse::<sqlx::postgres::PgConnectOptions>()
        .expect("unvalid database url")
        .ssl_mode(sqlx::postgres::PgSslMode::Prefer);
    tracing::info!(msg = "start connecting db", database = conn.get_database());
    sqlx::postgres::PgPoolOptions::new()
        .connect_with(conn)
        .await
        .unwrap_or_else(|_| panic!("connt to db error,db = {}", database_url))
}
