use sha2::{Digest, Sha256};
use std::{env, sync::OnceLock};

#[derive(Debug)]
pub struct Config {
    pub bot_token: String,
    pub bot_token_hash: [u8; 32],
    pub discord_webhook: String,
    pub sms_cb_pass: String,
    pub navasan_apikey: String,
    pub zarinpal: String,
}

impl Config {
    pub const RECORD_DIR: &'static str = "./record/";
    pub const TOKEN_ABC: &'static [u8] = b"!@#$%^&*_+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+";
    pub const TAX: f64 = 3.0; // 300%
}

pub fn config() -> &'static Config {
    static STATE: OnceLock<Config> = OnceLock::new();
    STATE.get_or_init(|| {
        let bot_token = env::var("TELOXIDE_TOKEN").unwrap();
        Config {
            bot_token_hash: Sha256::digest(&bot_token).into(),
            bot_token,
            discord_webhook: env::var("DISCORD_WEBHOOK").unwrap(),
            sms_cb_pass: env::var("SMS_CB_PASS").unwrap(),
            navasan_apikey: env::var("NAVASAN_APIKEY").unwrap(),
            zarinpal: env::var("ZARINPAL").unwrap(),
        }
    })
}
