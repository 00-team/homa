use serde::Deserialize;

use crate::config::{config, Config};

#[derive(Deserialize)]
pub struct ThoraUser {
    pub auth_date: i64,
    pub wallet: i64,
    pub in_hold: i64,
    pub admin: bool,
}

pub async fn user_get(uid: u64) -> anyhow::Result<Option<ThoraUser>> {
    let rq = reqwest::Client::new().get(Config::api("/api/user/"));
    Ok(config().api_auth(rq, uid).send().await?.json::<ThoraUser>().await.ok())
}

pub async fn star_price(user_id: u64) -> anyhow::Result<f64> {
    let rq = reqwest::Client::new().get(Config::api("/api/stars/price/"));
    Ok(config().api_auth(rq, user_id).send().await?.json::<f64>().await?)
}
