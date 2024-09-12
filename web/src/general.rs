use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use sqlx::{Pool, Sqlite};
use utoipa::ToSchema;

use crate::models::{AppErr, JsonStr};

#[derive(Serialize_tuple, Deserialize_tuple, Default, Clone)]
pub struct PriceValue {
    pub cost_api: f64,
    pub cost_buy: f64,
    pub count: i64,
    pub timestamp: i64,
}

type PriceData = HashMap<String, PriceValue>;

#[derive(Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct General {
    pub money_total: i64,
    pub money_gain: i64,
    pub money_loss: i64,
    pub rub_irr: i64,
    pub rub_irr_update: i64,
    pub usd_irr: i64,
    pub usd_irr_update: i64,
    pub star_tax: i64,
    pub phone_tax: i64,
    pub price_diff_total: i64,
    pub price_diff_count: i64,
    #[schema(value_type = HashMap<String, (f64, f64, i64, i64)>)]
    pub prices: JsonStr<PriceData>,
    pub prices_update: i64,
    pub disable_wallet: bool,
    pub disable_stars: bool,
    pub disable_phone: bool,
}

impl Default for General {
    fn default() -> Self {
        Self {
            money_total: 0,
            money_gain: 0,
            money_loss: 0,
            rub_irr: 0,
            rub_irr_update: 0,
            usd_irr: 0,
            usd_irr_update: 0,
            star_tax: 0,
            phone_tax: 0,
            price_diff_total: 0,
            price_diff_count: 0,
            prices: JsonStr(PriceData::new()),
            prices_update: 0,
            disable_wallet: false,
            disable_stars: false,
            disable_phone: false,
        }
    }
}

pub async fn general_get(pool: &Pool<Sqlite>) -> Result<General, AppErr> {
    let result = sqlx::query_as! {
        General,
        "select * from general"
    }
    .fetch_optional(pool)
    .await?;

    match result {
        Some(v) => Ok(v),
        None => {
            let _ = sqlx::query! {
                "insert into general default values"
            }
            .execute(pool)
            .await;

            Ok(General::default())
        }
    }
}

pub async fn general_set(
    pool: &Pool<Sqlite>, general: &General,
) -> Result<(), AppErr> {
    let result = sqlx::query_as! {
        General,
        "select * from general"
    }
    .fetch_optional(pool)
    .await?;

    match result {
        Some(_) => {
            sqlx::query! {"
                update general set
                money_total = ?,
                money_gain = ?,
                money_loss = ?,
                rub_irr = ?,
                rub_irr_update = ?,
                usd_irr = ?,
                usd_irr_update = ?,
                star_tax = ?,
                phone_tax = ?,
                prices = ?,
                price_diff_total = ?,
                price_diff_count = ?,
                prices_update = ?,
                disable_wallet = ?,
                disable_stars = ?,
                disable_phone = ?
            ",
                general.money_total,
                general.money_gain,
                general.money_loss,
                general.rub_irr,
                general.rub_irr_update,
                general.usd_irr,
                general.usd_irr_update,
                general.star_tax,
                general.phone_tax,
                general.prices,
                general.price_diff_total,
                general.price_diff_count,
                general.prices_update,
                general.disable_wallet,
                general.disable_stars,
                general.disable_phone
            }
            .execute(pool)
            .await?;

            Ok(())
        }
        None => {
            sqlx::query! {"
                insert into general(
                    money_total,
                    money_gain,
                    money_loss,
                    rub_irr,
                    rub_irr_update,
                    usd_irr,
                    usd_irr_update,
                    star_tax,
                    phone_tax,
                    prices,
                    price_diff_total,
                    price_diff_count,
                    prices_update,
                    disable_wallet,
                    disable_stars,
                    disable_phone
                ) values(?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
            ",
                general.money_total,
                general.money_gain,
                general.money_loss,
                general.rub_irr,
                general.rub_irr_update,
                general.usd_irr,
                general.usd_irr_update,
                general.star_tax,
                general.phone_tax,
                general.prices,
                general.price_diff_total,
                general.price_diff_count,
                general.prices_update,
                general.disable_wallet,
                general.disable_stars,
                general.disable_phone
            }
            .execute(pool)
            .await?;

            Ok(())
        }
    }
}
