#[macro_use]
extern crate log;

use pretty_env_logger;

use axum::{
    routing::{delete, get, post},
    Router,
};

use dotenvy::dotenv;

use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;
mod persistance;

use handlers::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // Test it out
    info!("such information");
    warn!("o_O");
    error!("much error");
    debug!("so debug");

    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    info!("Database connection pool created.");

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
    .await
    .unwrap();

    info!("Starting server...");

    axum::serve(listener, app).await.unwrap();
}
