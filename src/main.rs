use std::{collections::HashMap, env};

use axum::routing::get;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use lazy_static::lazy_static;
use lib::database::{model::*, schema::queue::dsl::queue};

mod lib;

type AccountId = i32;
lazy_static! {
    static ref queued_players: Mutex<HashMap<AccountId, SocketRef>> = Mutex::new(HashMap::new());
}

fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return MysqlConnection::establish(&database_url).expect("Could not connect to database");
}

struct MatchQualityConclusion {
    pub skill_gap: i32, // Difference in elo between highest and lowest rated player
    pub match_quality: MatchQuality,
    pub match_allowed: bool,
}

enum MatchQuality {
    Great,
    Good,
    Decent,
    Subpar,
    Terrible,
}

impl MatchQuality {
    fn is_allowed(minutes_in_mm: i32) -> bool {
        // TODO: implement this function
        true
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let (layer, io) = SocketIo::new_layer();

    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("startqueue", |s: SocketRef, Data::<Account>(account)| {
            // If player is not queued
            if let None = queued_players.blocking_lock().get(&account.id) {
                queued_players.blocking_lock().insert(account.id, s);
            }
        });
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // Enable CORS policy
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    loop {
        if let Ok(mut queues) = queue.load::<Queue>(&mut establish_connection()) {
            while let Some(queue_one) = &queues.pop() {
                for queue_two in &queues {}
            }
        }
    }
}

fn evaluate_match(queue_one: &Queue, queue_two: &Queue) -> MatchQualityConclusion {}
