use std::{env, sync::OnceLock};
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Config {
    pub bot_token_hash: [u8; 32],
}

pub fn config() -> &'static Config {
    static STATE: OnceLock<Config> = OnceLock::new();
    STATE.get_or_init(|| {
        let token = Sha256::digest(&env::var("TELOXIDE_TOKEN").unwrap());
        Config { bot_token_hash: token.into() }
    })
}
