use super::{AppErr, AppErrForbidden};
use crate::{config::config, utils::CutOff, AppState};
use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::{future::Future, ops, pin::Pin};
use utoipa::ToSchema;

enum Authorization {
    Bot { id: i64, token: String },
    User { id: i64, token: String },
}

impl TryFrom<&str> for Authorization {
    type Error = AppErr;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value.splitn(2, ' ');
        let key = tokens.next();
        let tokens = tokens.next().and_then(|v| Some(v.splitn(2, ':')));

        if key.is_none() || tokens.is_none() {
            return Err(AppErrForbidden("invalid authorization"));
        }

        let key = key.unwrap().to_string();
        let mut tokens = tokens.unwrap();
        let id = tokens.next().and_then(|v| v.parse::<i64>().ok());
        let token = tokens.next().and_then(|v| Some(v.to_string()));

        if id.is_none() || token.is_none() {
            return Err(AppErrForbidden("bad authorization"));
        }

        let id = id.unwrap();
        let token = token.unwrap();

        match key.as_str() {
            "bot" => Ok(Authorization::Bot { id, token }),
            "user" => Ok(Authorization::User { id, token }),
            _ => Err(AppErrForbidden("unknown authorization")),
        }
    }
}

impl TryFrom<&HttpRequest> for Authorization {
    type Error = AppErr;

    fn try_from(rq: &HttpRequest) -> Result<Self, Self::Error> {
        if let Some(value) = rq.headers().get("authorization") {
            return Authorization::try_from(value.to_str()?);
        }

        for hdr in rq.headers().get_all("cookie") {
            for cookie in hdr.as_bytes().split(|v| *v == b';') {
                let mut s = cookie.splitn(2, |v| *v == b'=');

                let k = s.next().and_then(|v| String::from_utf8(v.into()).ok());
                let v = s.next().and_then(|v| String::from_utf8(v.into()).ok());
                if k.is_none() || v.is_none() {
                    continue;
                }

                if k.unwrap().trim().to_lowercase() == "authorization" {
                    return Authorization::try_from(v.unwrap().as_str());
                }
            }
        }

        Err(AppErrForbidden("no authorization"))
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Default)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: Option<String>,
    pub auth_date: i64,
    pub wallet: i64,
    pub in_hold: i64,
    pub token: String,
    pub photo: bool,
    pub admin: bool,
    pub banned: bool,
}

pub struct Admin(pub User);

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

impl FromRequest for User {
    type Error = AppErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        let state = req.app_data::<Data<AppState>>().unwrap();
        let pool = state.sql.clone();
        let auth = Authorization::try_from(req);
        // let token = BearerAuth::from_request(req, pl);

        Box::pin(async move {
            let mut user = match auth? {
                Authorization::Bot { id, token } => {
                    if config().bot_auth != token {
                        return Err(AppErrForbidden("bad auth"));
                    }

                    sqlx::query_as! {
                        User, "select * from users where id = ?", id
                    }
                    .fetch_one(&pool)
                    .await?
                }
                Authorization::User { id, token } => {
                    let token = hex::encode(sha2::Sha512::digest(&token));
                    sqlx::query_as! {
                        User,
                        "select * from users where id = ? and token = ?",
                        id, token
                    }
                    .fetch_one(&pool)
                    .await?
                }
            };

            if user.banned {
                return Err(AppErrForbidden("banned"));
            }

            user.token.cut_off(32);
            Ok(user)
        })
    }
}

impl FromRequest for Admin {
    type Error = AppErr;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let user = User::from_request(req, payload);
        Box::pin(async {
            let user = user.await?;
            if !user.admin {
                return Err(AppErrForbidden("forbidden"));
            }

            Ok(Admin(user))
        })
    }
}
