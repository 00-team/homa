use sha2::{Digest, Sha256};
use std::{collections::HashSet, sync::OnceLock};

#[derive(Debug)]
pub struct Config {
    pub bot_auth: String,
    pub bot_token: String,
    pub bot_username: String,
    pub bot_token_hash: [u8; 32],
    pub sms_cb_pass: String,
    // pub navasan_apikey: String,
    pub zarinpal: String,
    pub admins: HashSet<u64>,
    pub trust: i64,
    pub bot_url: String,
}

impl Config {
    pub const STAR_COST: f64 = 0.015;
    pub const RECORD_DIR: &'static str = "./record/";
    pub const TOKEN_ABC: &'static [u8] = b"!@#$%^&*_+abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*_+";
    // pub const TAX: f64 = 3.0; // 300%
}

macro_rules! evar {
    ($name:literal) => {
        std::env::var($name).expect(concat!($name, " was not found in env"))
    };
}

pub fn config() -> &'static Config {
    static STATE: OnceLock<Config> = OnceLock::new();
    STATE.get_or_init(|| {
        let bot_token = evar!("TELOXIDE_TOKEN");
        let admins = serde_json::from_str::<Vec<u64>>(&evar!("ADMINS"))
            .expect("invalid ADMINS in .env");

        let admins = HashSet::<u64>::from_iter(admins);
        let bot_username = evar!("BOT_USERNAME");

        Config {
            admins,
            bot_token_hash: Sha256::digest(&bot_token).into(),
            bot_token,
            bot_url: format!("t.me/{}", bot_username),
            bot_username,
            sms_cb_pass: evar!("SMS_CB_PASS"),
            bot_auth: evar!("BOT_AUTH"),
            // navasan_apikey: env::var("NAVASAN_APIKEY").expect("env"),
            zarinpal: evar!("ZARINPAL"),
            trust: evar!("TRUST").parse::<i64>().expect("bad TRUST i64"),
        }
    })
}
