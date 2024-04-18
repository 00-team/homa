// use std::borrow::Cow;
// use sqlx::{encode::IsNull, sqlite::{SqliteArgumentValue, SqliteTypeInfo}, Sqlite};

use core::fmt;
use std::{future::Future, ops, pin::Pin};

use actix_web::{
    body::BoxBody,
    dev::Payload,
    error,
    http::StatusCode,
    web::{Data, Json},
    FromRequest, HttpRequest, HttpResponse, ResponseError,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::Digest;
use sqlx::{
    encode::IsNull,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo},
    Sqlite,
};
use utoipa::ToSchema;

use crate::{utils::CutOff, AppState};

#[derive(Deserialize)]
pub struct ListInput {
    pub page: u32,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Default)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub auth_date: i64,
    pub wallet: i64,
    pub in_hold: i64,
    pub token: String,
    pub photo: bool,
    pub admin: bool,
    pub banned: bool,
}

pub struct Admin(pub User);

pub type Response<T> = Result<Json<T>, Err>;

impl ops::Deref for Admin {
    type Target = User;

    fn deref(&self) -> &User {
        &self.0
    }
}

impl ops::DerefMut for Admin {
    fn deref_mut(&mut self) -> &mut User {
        &mut self.0
    }
}

fn parse_token(token: &str) -> Option<(i64, String)> {
    let mut token = token.splitn(2, ':');
    let id = match token.next() {
        Some(s) => match s.parse::<i64>() {
            Ok(v) => v,
            Err(_) => return None,
        },
        None => return None,
    };

    let token = match token.next() {
        Some(s) => s.to_string(),
        None => return None,
    };

    Some((id, token))
}

impl FromRequest for User {
    type Error = error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let state = req.app_data::<Data<AppState>>().unwrap();
        let pool = state.sql.clone();
        let token = BearerAuth::from_request(req, pl);

        Box::pin(async move {
            let (id, token) = match token.await {
                Ok(t) => match parse_token(t.token()) {
                    Some(t) => t,
                    None => return Err(error::ErrorForbidden("invalid token")),
                },
                Err(e) => return Err(e.into()),
            };

            let token = hex::encode(sha2::Sha512::digest(&token));

            let result = sqlx::query_as! {
                User,
                "select * from users where id = ? and token = ?",
                id, token
            }
            .fetch_one(&pool)
            .await;

            match result {
                Ok(mut user) => {
                    if user.banned {
                        return Err(error::ErrorForbidden("banned"));
                    }

                    user.token.cut_off(32);
                    Ok(user)
                }
                Err(_) => Err(error::ErrorForbidden("user not found")),
            }
        })
    }
}

impl FromRequest for Admin {
    type Error = error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let user = User::from_request(req, payload);
        Box::pin(async {
            let user = user.await?;
            if !user.admin {
                return Err(error::ErrorForbidden("invalid admin"));
            }

            Ok(Admin(user))
        })
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Transaction {
    pub id: i64,
    pub user: i64,
    pub kind: i64,   // in OR out | withdrawl OR deposit
    pub status: i64, // success | failed | in progress
    pub amount: i64,
    pub vendor_order_id: Option<String>,
    pub vendor_track_id: Option<i64>,
    pub card_number: Option<String>,
    pub hashed_card_number: Option<String>,
    pub date: Option<i64>,
    pub bank_track_id: Option<i64>,
}

#[derive(Debug, Deserialize, Default)]
pub struct JsonStr<T>(pub T);

// impl<T> JsonStr<T> {
//     pub fn into_inner(self) -> T {
//         self.0
//     }
// }

impl<T> ops::Deref for JsonStr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for JsonStr<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Serialize> Serialize for JsonStr<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'q, T: Serialize> sqlx::Encode<'q, Sqlite> for JsonStr<T> {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> IsNull {
        let result = serde_json::to_string(&self.0).unwrap_or("{}".to_string());
        buf.push(SqliteArgumentValue::Text(result.into()));

        IsNull::No
    }
}

impl<T> sqlx::Type<Sqlite> for JsonStr<T> {
    fn type_info() -> SqliteTypeInfo {
        <&str as sqlx::Type<Sqlite>>::type_info()
    }
}

impl<T: DeserializeOwned + Default> From<String> for JsonStr<T> {
    fn from(value: String) -> Self {
        Self(serde_json::from_str::<T>(&value).unwrap_or(T::default()))
    }
}

#[derive(Serialize, Debug)]
pub struct Err {
    status: u16,
    message: String,
}

impl fmt::Display for Err {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for Err {
    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .body(serde_json::to_string::<Err>(self).unwrap())
    }
}

impl From<sqlx::Error> for Err {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self {
                status: 404,
                message: "not found".to_string(),
            },
            _ => Self {
                status: 500,
                message: "database error".to_string(),
            },
        }
    }
}
