use std::collections::HashMap;

use sqlx::{Pool, Sqlite};

use crate::models::{AppErr, JsonStr};

type PriceData = HashMap<String, i64>;

#[derive(sqlx::FromRow)]
pub struct General {
    pub available_money: i64,
    pub total_money: i64,
    pub rub_irr: i64,
    pub rub_irr_update: i64,
    pub price_diff_total: i64,
    pub price_diff_count: i64,
    pub prices: JsonStr<PriceData>,
    pub prices_update: i64,
}

impl Default for General {
    fn default() -> Self {
        Self {
            available_money: 0,
            total_money: 0,
            rub_irr: 0,
            rub_irr_update: 0,
            price_diff_total: 0,
            price_diff_count: 0,
            prices: JsonStr(PriceData::new()),
            prices_update: 0,
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
            sqlx::query! {
                "update general set available_money = ?, total_money = ?,
                rub_irr = ?, rub_irr_update = ?, price_diff_total = ?,
                price_diff_count = ?, prices = ?, prices_update = ?",
                general.available_money, general.total_money, general.rub_irr,
                general.rub_irr_update, general.price_diff_total,
                general.price_diff_count, general.prices, general.prices_update
            }
            .execute(pool)
            .await?;

            Ok(())
        }
        None => {
            sqlx::query! {
                "insert into general(available_money, total_money, rub_irr,
                rub_irr_update, price_diff_total, price_diff_count, prices,
                prices_update) values(?,?,?,?,?,?,?,?)",
                general.available_money, general.total_money, general.rub_irr,
                general.rub_irr_update, general.price_diff_total,
                general.price_diff_count, general.prices, general.prices_update
            }
            .execute(pool)
            .await?;

            Ok(())
        }
    }
}
