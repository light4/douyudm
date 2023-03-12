use clap::Parser;

pub const HEARBEAT_INTERVAL: u64 = 45;

pub const MSG_LIVE_ON: &str = "主播正在直播";
pub const MSG_LIVE_OFF: &str = "主播没有直播";
pub const MSG_ROOM_RSS: &str = "房间开播提醒";
pub const MSG_BC_BUY_DESERVE: &str = "赠送酬勤通知";
pub const MSG_SSD: &str = "超级弹幕";
pub const MSG_ROOM_SPBC: &str = "房间内礼物广播";

pub const WSS_URL: &str = "wss://danmuproxy.douyu.com";

// port range 8501..=8506
pub fn random_wss_url() -> String {
    let port = 8500 + fastrand::usize(1..=6);
    // let port = 8503;
    format!("{}:{}/", WSS_URL, port)
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// douyu room id
    pub room_id: u64,
}
