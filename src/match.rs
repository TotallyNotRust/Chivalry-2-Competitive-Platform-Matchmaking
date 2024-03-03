use serde::Serialize;
use crate::{Match, Account};

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MatchWithPlayers {
    pub id: i64,
    pub gamemode: i32,
    pub players: Vec<Account>
}

impl MatchWithPlayers {
    pub fn from_match(_match: &Match, players: Vec<Account>) -> MatchWithPlayers {
        MatchWithPlayers {
            id: _match.id,
            gamemode: _match.gamemode,
            players: players,
        }
    }
}


