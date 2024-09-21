use core::panic;
use std::{env, sync::OnceLock};

use reqwest::{RequestBuilder, Url};
use teloxide::types::{LoginUrl, UserId};

#[derive(Debug)]
pub struct Config {
    pub dev: UserId,
    pub admins: Vec<UserId>,
    pub bot_username: String,
    pub login_url: LoginUrl,
    pub bot_auth: String,
    pub orders_url: Url,
}

#[cfg(debug_assertions)]
impl Config {
    pub const HOST: &'static str = "http://localhost:7000";
}

#[cfg(not(debug_assertions))]
impl Config {
    pub const HOST: &'static str = "https://thora.dozar.bid";
}

impl Config {
    pub fn api(url: &str) -> String {
        format!("{}{url}", Self::HOST)
    }

    pub fn api_auth(&self, rq: RequestBuilder, uid: u64) -> RequestBuilder {
        rq.header("authorization", format!("bot {uid}:{}", self.bot_auth))
    }
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
        let orders_url = Url::parse("https://thora.dozar.bid/orders/")
            .expect("invalid orders url");
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
            orders_url,
        }
    })
}
