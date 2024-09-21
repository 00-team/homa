use serde::Deserialize;

use crate::config::{config, Config};

#[derive(Deserialize)]
pub struct ThoraUser {
    pub auth_date: i64,
    pub wallet: i64,
    pub in_hold: i64,
    pub admin: bool,
}

pub async fn user_get(user_id: u64) -> anyhow::Result<Option<ThoraUser>> {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("{}/api/user/", Config::API))
        .header("authorization", format!("bot {user_id}:{}", config().bot_auth))
        .send()
        .await?;

    Ok(res.json::<ThoraUser>().await.ok())
}
