use warp::{
    Filter,
    http::Method, 
    Rejection, 
    Reply, 
    http::StatusCode
};
use std::{collections::HashMap};
use tracing_subscriber::fmt::format::FmtSpan;
use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;
mod routes;
mod types;
mod error;
mod database;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPool::connect(&database_url)
    .await
    .expect("Couldn't connect to database");
    
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_|  "corona-map=info,warp=error".to_owned());
    // let store = store::Store::new();

    let db_filter = warp::any().map(move ||pool.clone());
    // let store_filter = warp::any().map(move||store.clone());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::POST, Method::DELETE]);

        let get_events = warp::get()
        .and(warp::path("events"))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and_then(routes::handler::get_events)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get_events request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )})
        );


        let add_event = warp::post()
        .and(warp::path("events"))
        .and(warp::path::end())
        .and(db_filter.clone())
        .and(warp::body::json())
        .and_then(routes::handler::add_event);

        // let delete_event = warp::delete()
        // .and(warp::path("events"))
        // .and(warp::path::param::<String>())
        // .and(warp::path::end())
        // .and(store_filter.clone())
        // .and(warp::body::json())
        // .and_then(routes::handler::delete_event);

    let routes = get_events.or(add_event).with(cors).with(warp::trace::request()).recover(error::return_error);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
    
}