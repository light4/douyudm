use std::{thread, time::Duration};

use color_eyre::Result;
use serde_json::{json, Value};

use crate::config::{Cli, HEARBEAT_INTERVAL};

#[derive(Debug)]
pub struct Client {
    pub room_id: u64,
}

impl Client {
    pub fn new(cli: &Cli) -> Self {
        Self {
            room_id: cli.room_id,
        }
    }

    pub fn init_websocket(&self) -> Result<()> {
        todo!()
    }

    pub fn send(&self, msg: Value) -> Result<()> {
        todo!()
    }

    pub fn login(&self) -> Result<()> {
        let msg = json!({"type": "loginreq", "roomid": self.room_id});
        self.send(msg)
    }

    pub fn join_group(&self) -> Result<()> {
        let msg = json!({"type": "joingroup", "rid": self.room_id, "gid": -9999});
        self.send(msg)
    }

    pub fn heartbeat(&self) -> Result<()> {
        let msg = json!({"type": "mrkl"});
        thread::sleep(Duration::from_secs(HEARBEAT_INTERVAL));
        self.send(msg)
    }

    pub fn handle_message(&self, data: Value) -> Result<()> {
        todo!()
    }
}
