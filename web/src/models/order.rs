use super::{sql_enum, AppErr};
use crate::AppState;
use actix_web::dev::Payload;
use actix_web::{
    web::{Data, Path},
    FromRequest, HttpRequest,
};
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};
use utoipa::ToSchema;

sql_enum! {
    pub enum OrderStatus {
        Wating,
        Refunded,
        Done,
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Default)]
pub struct PhoneOrder {
    pub id: i64,
    pub user: i64,
    pub status: OrderStatus,
    pub activation_id: i64,
    pub phone: String,
    pub cost: i64,
    pub operator: String,
    pub datetime: String,
    pub country: String,
    pub service: String,
}

impl FromRequest for PhoneOrder {
    type Error = AppErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        #[derive(Deserialize)]
        struct OrderPath {
            poid: i64,
        }
        let path = Path::<OrderPath>::extract(req);
        let state = req.app_data::<Data<AppState>>().unwrap();
        let pool = state.sql.clone();

        Box::pin(async move {
            let path = path.await?;
            let order = sqlx::query_as! {
                PhoneOrder,
                "select * from phone_orders where id = ?",
                path.poid
            }
            .fetch_one(&pool)
            .await?;

            Ok(order)
        })
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Default)]
pub struct StarOrder {
    pub id: i64,
    pub user: i64,
    pub status: OrderStatus,
    pub amount: i64,
    pub cost: i64,
    pub timestamp: i64,
    pub hash: Option<String>,
}

impl FromRequest for StarOrder {
    type Error = AppErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        #[derive(Deserialize)]
        struct OrderPath {
            soid: i64,
        }
        let path = Path::<OrderPath>::extract(req);
        let state = req.app_data::<Data<AppState>>().unwrap();
        let pool = state.sql.clone();

        Box::pin(async move {
            let path = path.await?;
            let order = sqlx::query_as! {
                StarOrder,
                "select * from star_orders where id = ?",
                path.soid
            }
            .fetch_one(&pool)
            .await?;

            Ok(order)
        })
    }
}
