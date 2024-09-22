use crate::{
    config::{config, Config},
    utils::toman,
};
use indoc::formatdoc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ThoraErr {
    status: u16,
    message: String,
}

impl From<ThoraErr> for String {
    fn from(value: ThoraErr) -> Self {
        formatdoc! {"
            خطا ❌

            {}

            کد: {}

            --- thora ---
        ", value.message, value.status}
    }
}

#[derive(Deserialize)]
pub struct ThoraUser {
    pub auth_date: i64,
    pub wallet: i64,
    pub in_hold: i64,
    pub admin: bool,
    pub username: Option<String>,
}

pub async fn user_get(uid: u64) -> anyhow::Result<Option<ThoraUser>> {
    let rq = reqwest::Client::new().get(Config::api("/api/user/"));
    Ok(config().api_auth(rq, uid).send().await?.json::<ThoraUser>().await.ok())
}

pub async fn star_price(user_id: u64) -> anyhow::Result<f64> {
    let rq = reqwest::Client::new().get(Config::api("/api/stars/price/"));
    Ok(config().api_auth(rq, user_id).send().await?.json::<f64>().await?)
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Wating,
    Refunded,
    Done,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dpy = match self {
            Self::Done => "تکمیل شده ✅",
            Self::Refunded => "ریفاند شده ❌",
            Self::Wating => "درحال تکمیل ⏳",
        };
        write!(f, "{}", dpy)
    }
}

#[derive(Deserialize)]
pub struct StarOrder {
    pub id: i64,
    pub user: i64,
    pub status: OrderStatus,
    pub amount: i64,
    pub cost: i64,
    pub timestamp: i64,
    pub hash: Option<String>,
}

impl From<StarOrder> for String {
    fn from(value: StarOrder) -> Self {
        formatdoc! {"
            سفارش استار ⭐
            وضعیت: {}
            تعداد: {}
            قیمت: {}

            --- thora ---
        ",
            value.status, value.amount, toman(value.cost),
        }
    }
}

pub async fn stars_buy(
    uid: u64, amount: u64,
) -> anyhow::Result<Result<StarOrder, ThoraErr>> {
    #[derive(Serialize)]
    struct Body {
        amount: u64,
    }
    let rq = reqwest::Client::new().post(Config::api("/api/stars/buy/"));
    let res = config().api_auth(rq, uid).json(&Body { amount }).send().await?;
    Ok(if res.status() == reqwest::StatusCode::OK {
        Ok(res.json::<StarOrder>().await?)
    } else {
        Err(res.json::<ThoraErr>().await?)
    })
}
