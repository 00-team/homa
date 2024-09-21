use core::panic;
use std::{env, sync::OnceLock};

use reqwest::Url;
use teloxide::types::{LoginUrl, UserId};

#[derive(Debug)]
pub struct Config {
    pub dev: UserId,
    pub admins: Vec<UserId>,
    pub bot_username: String,
    pub login_url: LoginUrl,
    pub bot_auth: String,
}

#[cfg(debug_assertions)]
impl Config {
    pub const API: &'static str = "http://localhost:7000";
}

#[cfg(not(debug_assertions))]
impl Config {
    pub const API: &'static str = "https://thora.dozar.bid";
}

macro_rules! evar {
    ($name:literal) => {
        env::var($name).expect(concat!($name, " was not found in env"))
    };
}

macro_rules! env_num {
    ($name:literal, $ty:ty) => {
        env::var($name)
            .expect(concat!($name, " was not found in env"))
            .parse::<$ty>()
            .expect(concat!($name, " is not a number"))
    };
}

pub fn config() -> &'static Config {
    static STATE: OnceLock<Config> = OnceLock::new();
    STATE.get_or_init(|| {
        let login_url = LoginUrl {
            url: Url::parse("https://thora.dozar.bid/api/auth/login-telegram/")
                .expect("invalid login url"),
            forward_text: Some("some forward text".to_string()),
            bot_username: None,
            request_write_access: Some(true),
        };

        let dev = UserId(env_num!("DEVELOPER", u64));

        let bot_username = evar!("BOT_USERNAME");
        if bot_username.starts_with("@") {
            panic!("BOT_USERNAME must NOT start with @");
        }

        Config {
            login_url,
            dev,
            admins: serde_json::from_str(&evar!("ADMINS")).expect("bad ADMINS"),
            bot_username,
            bot_auth: evar!("BOT_AUTH"),
        }
    })
}
