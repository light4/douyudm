mod client;
mod config;
mod msg;
mod packet;

use std::time::{Duration, Instant};

use clap::Parser;
use color_eyre::Result;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::{
        client::IntoClientRequest,
        extensions::DeflateConfig,
        protocol::{Message, WebSocketConfig},
    },
    WebSocketStream,
};

use crate::{config::random_wss_url, msg::deserialize};

pub async fn real_main() -> Result<()> {
    let cli = config::Cli::parse();
    let client = client::Client::new(&cli);

    let wss_url = random_wss_url();
    let request = wss_url.into_client_request()?;
    let conn_config = Some(WebSocketConfig {
        compression: Some(DeflateConfig::default()),
        ..WebSocketConfig::default()
    });

    let (mut ws_stream, _) = connect_async_with_config(request, conn_config).await?;

    let msg = json!({"type": "loginreq", "roomid": client.room_id});
    let encoded = packet::encode(&msg::serialize(&msg));
    ws_stream.send(Message::binary(encoded)).await?;

    let msg = json!({"type": "joingroup", "rid": client.room_id, "gid": -9999});
    let encoded = packet::encode(&msg::serialize(&msg));
    ws_stream.send(Message::binary(encoded)).await?;

    let mut last_tick = Instant::now();
    loop {
        if last_tick.elapsed() >= Duration::from_secs(config::HEARBEAT_INTERVAL) {
            let msg = json!({"type": "mrkl"});
            let encoded = packet::encode(&msg::serialize(&msg));
            ws_stream.send(Message::binary(encoded)).await?;
            last_tick = Instant::now();
        }

        let msg = ws_stream.next().await.expect("Can't fetch case count")?;
        let resp = packet::decode(msg.into_data());
        let deserialized = deserialize(&resp);
        if let Some(obj) = deserialized.as_object() {
            let res_type = obj.get("type").and_then(|i| i.as_str()).unwrap_or_default();
            if res_type == "loginres" {
                println!("登录成功");
            } else if res_type == "chatmsg" {
                let level = obj.get("level").unwrap().as_str().unwrap_or("0");
                let nickname = obj.get("nn").unwrap().as_str().unwrap_or_default();
                let txt = obj.get("txt").unwrap().as_str().unwrap_or_default();
                println!("[{:2}][{}] {}", level, nickname, txt);
            }
        }
    }

    // ws_stream.close(None).await?;
    // Ok(())
}
