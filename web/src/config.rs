use sha2::{Digest, Sha256};
use std::{env, sync::OnceLock};

#[derive(Debug)]
pub struct Config {
    pub bot_token_hash: [u8; 32],
    pub discord_webhook: String,
    pub sms_cb_pass: String,
}

impl Config {
    pub const RECORD_DIR: &'static str = "./record/";
    pub const TOKEN_ABC: &'static [u8] = b"!@#$%^&*_+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+";
}

pub fn config() -> &'static Config {
    static STATE: OnceLock<Config> = OnceLock::new();
    STATE.get_or_init(|| {
        let token = Sha256::digest(&env::var("TELOXIDE_TOKEN").unwrap());
        Config {
            bot_token_hash: token.into(),
            discord_webhook: env::var("DISCORD_WEBHOOK").unwrap(),
            sms_cb_pass: env::var("SMS_CB_PASS").unwrap(),
        }
    })
}
