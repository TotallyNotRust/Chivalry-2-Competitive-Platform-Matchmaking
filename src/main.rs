use std::sync::Arc;
use std::{collections::HashMap, env};
use tokio::sync::RwLock;

use axum::routing::get;
use diesel::{
    insert_into, sql_function, sql_query, sql_types::Integer, BoolExpressionMethods, Connection,
    ExpressionMethods, MysqlConnection, QueryDsl, RunQueryDsl,
};
use dotenv::dotenv;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use lazy_static::lazy_static;
use lib::database::{
    model::*,
};

use crate::{
    lib::database::model::{Account, NewToken, Token},
    lib::database::schema::{
        account::dsl::{account, id as account_id},
        tokens::dsl::{id as token_id, invalidated, token, tokens},
    },
};

use crate::queue::Queue;

mod lib;
mod queue;

type AccountId = i32;
lazy_static! {
    static ref QUEUED_PLAYERS: RwLock<Vec<Queue>> = RwLock::new(vec![]);
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

    sql_function!(
        #[aggregate]
        #[sql_name="LAST_INSERT_ID"]
        fn last_id() -> Integer
    );

    env_logger::init();

    let (layer, io) = SocketIo::new_layer();

    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("start-queue", handle_player_join_queue);
    });

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello!" }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive()) // Enable CORS policy
                .layer(layer),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Opened listener on port 3001");
    axum::serve(listener, app).await.unwrap();

    loop {
        let mut queue_lock = QUEUED_PLAYERS.read().await;
        if let queue = *queue_lock {
            drop(queue_lock);
            while let Some(queue_one) = &queue.pop() {
                for queue_two in &queue {
                    if queue_one.gamemode == queue_two.gamemode {
                        let match_qual = evaluate_match(queue_one, queue_two);

                        if match_qual.match_allowed {
                            println!("Found match");

                        }
                    }
                }
            }
        } else {
            drop(queue_lock);
        }
    }
}

fn evaluate_match(queue_one: &Queue, queue_two: &Queue) -> MatchQualityConclusion {
    return MatchQualityConclusion {
        skill_gap: 500,
        match_quality: MatchQuality::Subpar,
        match_allowed: false,
    };
}

pub fn token_to_account(_token: &str) -> Option<(Account, Token)> {
    if let Some(token_obj) = tokens
        .filter(token.eq(_token).and(invalidated.eq(false)))
        .load::<Token>(&mut establish_connection())
        .unwrap()
        .first()
    {
        if let Some(acc) = account
            .filter(account_id.eq(token_obj.account_id))
            .load::<Account>(&mut establish_connection())
            .unwrap()
            .first()
        {
            println!("Found account from token");
            return Some((acc.to_owned(), token_obj.to_owned()));
        }
    }
    println!("Cant find account from token");
    return None;
}

async fn handle_player_join_queue(s: SocketRef, _account_token: Data<String>) {
    let Data(account_token) = _account_token;
    let _account = token_to_account(&account_token);

    if _account.is_none() {
        return;
    }

    let (_account, _token) = _account.unwrap();
    println!("Player with id {} wants to join the queue", _account.id);
    // If player is not queued
    let queued_players_ref = QUEUED_PLAYERS.read().await;
    println!("wefhuwfuiwefhuiwefhweuifhwefuiweh");
    if (*queued_players_ref).iter().any(|x: &Queue| x.account.id == _account.id) {
        drop(queued_players_ref);
        println!("Player with id {} joined queue", _account.id);
        s.emit("joined-queue", "").ok();

        {
            let mut queue_ref = QUEUED_PLAYERS.write().await;
            queue_ref.push(Queue {
                gamemode: 1,
                account: _account.clone(),
                socket: s,
            });

            drop(queue_ref);
            println!("Added Player with id {} to queue", _account.id);
        };
    } else {
        drop(queued_players_ref);
        println!(
            "Player with id {} could not join queue, as they already have",
            _account.id
        );
        s.emit("join-failed", "You are already in queue").ok();
    }
    println!("fIN");
}
