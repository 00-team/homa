use sqlx::{Pool, Sqlite};

use crate::models::AppErr;

#[derive(sqlx::FromRow)]
pub struct General {
    pub available_money: i64,
    pub total_money: i64,
    pub rub_irr: i64,
    pub rub_irr_update: i64,
    // pub vnum_prices: String,
    // pub vnum_update: i64,
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

            Ok(General {
                available_money: 0,
                total_money: 0,
                rub_irr: 0,
                rub_irr_update: 0,
            })
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
                "update general set
                available_money = ?, total_money = ?,
                rub_irr = ?, rub_irr_update = ?",
                general.available_money, general.total_money,
                general.rub_irr, general.rub_irr_update
            }
            .execute(pool)
            .await?;

            Ok(())
        }
        None => {
            sqlx::query! {
                "insert into general values(?, ?, ?, ?)",
                general.available_money, general.total_money,
                general.rub_irr, general.rub_irr_update
            }
            .execute(pool)
            .await?;

            Ok(())
        }
    }
}
