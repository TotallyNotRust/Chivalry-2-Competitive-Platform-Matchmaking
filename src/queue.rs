use socketioxide::extract::SocketRef;

use crate::lib::database::model::Account;

pub struct Queue {
    pub gamemode: i16,
    pub account: Account,
    pub socket: SocketRef,
}


