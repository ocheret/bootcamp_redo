// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions
use dotenvy;
use log::{debug, error, info, warn};
use pretty_env_logger;
use sqlx::postgres::PgPoolOptions;
use models::QuestionDetail;

use axum::{
    routing::{delete, get, post},
    Router,
};

mod handlers;
mod models;

use handlers::*;

#[tokio::main]
async fn main() {
    // TODO: Initialize pretty_env_logger
    pretty_env_logger::init();

    // Test it out
    info!("such information");
    warn!("o_O");
    error!("much error");
    debug!("so debug");

    // TODO: Initialize dotenv
    dotenvy::dotenv().expect("Failed to load .env file");

    // Use dotenv to get the database url.
    let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create connection pool");

    info!("Connected to database");

    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    // let pool = todo!();

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    // let recs = todo!();
    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch questions");

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro
    info!("Questions - {:?}", recs);

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

    axum::serve(listener, app).await.unwrap();
}
