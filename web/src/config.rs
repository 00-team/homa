use sha2::{Digest, Sha256};
use std::{collections::HashSet, env, sync::OnceLock};

#[derive(Debug)]
pub struct Config {
    pub bot_token: String,
    pub bot_token_hash: [u8; 32],
    pub discord_webhook: String,
    pub sms_cb_pass: String,
    pub navasan_apikey: String,
    pub zarinpal: String,
    pub admins: HashSet<u64>,
}

impl Config {
    pub const RECORD_DIR: &'static str = "./record/";
    pub const TOKEN_ABC: &'static [u8] = b"!@#$%^&*_+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+";
    pub const TAX: f64 = 3.0; // 300%
}

pub fn config() -> &'static Config {
    static STATE: OnceLock<Config> = OnceLock::new();
    STATE.get_or_init(|| {
        let bot_token = env::var("TELOXIDE_TOKEN").expect("env: T_TOKEN");
        let admins = serde_json::from_str::<Vec<u64>>(
            &env::var("ADMINS").expect("env: ADMINS"),
        )
        .expect("invalid ADMINS in .env");

        let admins = HashSet::<u64>::from_iter(admins);

        Config {
            admins,
            bot_token_hash: Sha256::digest(&bot_token).into(),
            bot_token,
            discord_webhook: env::var("DISCORD_WEBHOOK").expect("env"),
            sms_cb_pass: env::var("SMS_CB_PASS").expect("env"),
            navasan_apikey: env::var("NAVASAN_APIKEY").expect("env"),
            zarinpal: env::var("ZARINPAL").expect("env"),
        }
    })
}
