use serde::{Serialize, Deserialize};
use crate::{Account};
use chrono::NaiveDateTime;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub from: Account,
    pub message: String,
}


