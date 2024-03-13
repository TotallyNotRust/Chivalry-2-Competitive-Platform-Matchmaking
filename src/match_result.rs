use serde::Deserialize;

use crate::lib::database::model::Account;

#[derive(Deserialize)]
pub struct MatchResult {
    pub match_id: i32,
    pub score: Score,
    pub approved: bool,
    pub disputed_needs_admin_conf: bool,
}

#[derive(Deserialize)]
struct Score {
    pub player1: Account,
    pub player2: Account,
    pub player1_score: i32,
    pub player2_score: i32,
}