use std::sync::Arc;

use socketioxide::extract::SocketRef;

use crate::Account;

#[derive(Clone, Debug, PartialEq)]
pub struct Queue {
    pub gamemode: i16,
    pub account: Account,
    pub socket: Arc<SocketRef>,
}


