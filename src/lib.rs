mod client;
mod config;
mod msg;
mod packet;

use std::time::{Duration, Instant};

use crate::{config::random_wss_url, msg::deserialize};
use color_eyre::Result;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use serde_json::Value;
use serde_json::{json, Deserializer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::client::IntoClientRequest,
    tungstenite::protocol::Message,
    tungstenite::{extensions::DeflateConfig, protocol::WebSocketConfig},
    WebSocketStream,
};

pub async fn real_main() -> Result<()> {
    let roomid = if let Some(room) = std::env::args().nth(1) {
        room.parse().unwrap_or(9999)
    } else {
        9999
    };

    let wss_url = random_wss_url();
    let request = wss_url.into_client_request()?;
    let conn_config = Some(WebSocketConfig {
        compression: Some(DeflateConfig::default()),
        ..WebSocketConfig::default()
    });

    let (mut ws_stream, _) = connect_async_with_config(request, conn_config).await?;

    let msg = json!({"type": "loginreq", "roomid": roomid});
    let encoded = packet::encode(&msg::serialize(&msg));
    ws_stream.send(Message::binary(encoded)).await?;

    let msg = json!({"type": "joingroup", "rid": roomid, "gid": -9999});
    let encoded = packet::encode(&msg::serialize(&msg));
    ws_stream.send(Message::binary(encoded)).await?;

    let mut last_tick = Instant::now();
    loop {
        let msg = ws_stream.next().await.expect("Can't fetch case count")?;
        let resp = packet::decode(msg.into_data());
        let deserialized = deserialize(&resp);
        if let Some(obj) = deserialized.as_object() {
            let res_type = obj.get("type").unwrap().as_str().unwrap_or_default();
            if res_type == "loginres" {
                println!("登录成功");
            } else if res_type == "chatmsg" {
                let level = obj.get("level").unwrap().as_str().unwrap_or("0");
                let nickname = obj.get("nn").unwrap().as_str().unwrap_or_default();
                let txt = obj.get("txt").unwrap().as_str().unwrap_or_default();
                println!("[{:2}][{}]: {}", level, nickname, txt);
            }
        }

        if last_tick.elapsed() >= Duration::from_secs(45) {
            let msg = json!({"type": "mrkl"});
            let encoded = packet::encode(&msg::serialize(&msg));
            ws_stream.send(Message::binary(encoded)).await?;
            last_tick = Instant::now();
        }
    }

    ws_stream.close(None).await?;
    Ok(())
}
