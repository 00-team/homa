use actix_web::web::{Data, Json, Path, Query, Redirect};
use actix_web::{get, post, HttpResponse, Scope, http::header};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

use crate::config::config;
use crate::docs::UpdatePaths;
use crate::models::message::Message;
use crate::models::order::Order;
use crate::models::transaction::{Transaction, TransactionStatus, TransactionKind};
use crate::models::user::User;
use crate::models::{AppErr, AppErrBadRequest, ListInput, Response};
use crate::{utils, AppState};

#[derive(OpenApi)]
#[openapi(
    tags((name = "api::user")),
    paths(
        user_get, user_deposit, user_transactions, user_orders,
        user_messages, user_message_seen, user_messages_unseen_count,
    ),
    components(schemas()),
    servers((url = "/user")),
    modifiers(&UpdatePaths)
)]
pub struct ApiDoc;

#[utoipa::path(get, responses((status = 200, body = User)))]
/// Get
#[get("/")]
async fn user_get(user: User) -> Response<User> {
    Ok(Json(user))
}

#[derive(Deserialize, IntoParams)]
struct DepositQuery {
    amount: u64,
}

#[derive(Deserialize, Debug)]
struct ZarinpalResponse<T> {
    data: T,
}

#[utoipa::path(
    get,
    params(DepositQuery),
    responses((status = 200, body = String))
)]
/// Deposit
#[get("/deposit/")]
async fn user_deposit(
    user: User, q: Query<DepositQuery>, state: Data<AppState>,
) -> Redirect {
    let allowed = 50_000_000 - user.wallet;
    if allowed < 50_000 {
        return Redirect::to("/?error=wallet is maxed out");
        // return Err(AppErrBadRequest("wallet is maxed out"));
    }

    let amount = q.amount as i64;
    let amount = amount.max(50_000).min(allowed);
    let now = utils::now();

    #[derive(Serialize)]
    struct Data {
        merchant_id: String,
        amount: i64,
        description: String,
        callback_url: String,
    }

    let client = awc::Client::new();
    let result = client
        .post("https://payment.zarinpal.com/pg/v4/payment/request.json")
        .send_json(&Data {
            merchant_id: config().zarinpal.clone(),
            amount,
            description: format!("{} deposit", user.name),
            callback_url: "https://thora.dozar.bid/api/user/zcb/".to_string(),
        })
        .await;

    let mut result = if let Ok(r) = result {
        r
    } else {
        log::error!("zarinpal result: {result:?}");
        return Redirect::to("/?error=zarinpal failed");
    };

    #[derive(Debug, Deserialize)]
    struct RsData {
        code: i64,
        authority: String,
    }

    let data = result.json::<ZarinpalResponse<RsData>>().await; //?.data;
    let data = if let Ok(d) = data {
        d.data
    } else {
        return Redirect::to("/?error=درخواست پرداخت به مشکل خورد");
    };
    if data.code != 100 {
        log::error!("payment failed: {:?}", result.body().await);
        return Redirect::to("/?error=درخواست پرداخت به مشکل خورد");
    }

    let dbr = sqlx::query! {
        "insert into transactions(user, amount, kind, status, timestamp, authority)
        values(?, ?, ?, ?, ?, ?)",
        user.id, amount, TransactionKind::In, TransactionStatus::InProgress,
        now, data.authority
    }
    .execute(&state.sql)
    .await;

    if dbr.is_err() {
        return Redirect::to("/?error=db error");
    }

    Redirect::to(format!(
        "https://www.zarinpal.com/pg/StartPay/{}",
        data.authority
    ))
}


#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "PascalCase")]
struct ZcbQuery {
    authority: String,
    status: String,
}

#[utoipa::path(get, params(ZcbQuery))]
/// Zarinpal Callback
#[get("/zcb/")]
async fn zcb(
    user: User, q: Query<ZcbQuery>, state: Data<AppState>,
) -> Redirect {
    // let response = HttpResponse::Found()
    //     .insert_header((header::LOCATION, "/"))
    //     .finish();

    let response = Redirect::to("/");

    let transaction = sqlx::query_as! {
        Transaction,
        "select * from transactions where
        authority = ? and user = ? and status = ?",
        q.authority, user.id,
        TransactionStatus::InProgress
    }
    .fetch_one(&state.sql)
    .await;

    let is_ok = q.status.to_lowercase() == "ok";
    if !is_ok || transaction.is_err() {
        return response;
    }
    let transaction = transaction.unwrap();
    let wallet = user.wallet + transaction.amount;

    // if cfg!(debug_assertions) {
    //     let result = sqlx::query! {
    //         "update users set wallet = ? where id = ?",
    //         wallet, user.id
    //     }
    //     .execute(&state.sql)
    //     .await;
    //
    //     if result.is_err() {
    //         return response;
    //     }
    //
    //     let _ = sqlx::query! {
    //         "update transactions set status = ? where id = ?",
    //         TransactionStatus::Success, transaction.id
    //     }
    //     .execute(&state.sql)
    //     .await;
    //
    //     return response;
    // }

    #[derive(Serialize)]
    struct Data {
        merchant_id: String,
        amount: i64,
        authority: String,
    }

    let client = awc::Client::new();
    let result = client
        .post("https://payment.zarinpal.com/pg/v4/payment/verify.json")
        .send_json(&Data {
            merchant_id: config().zarinpal.clone(),
            amount: transaction.amount,
            authority: q.authority.clone(),
        })
        .await;

    let failed = || {
        sqlx::query! {
            "update transactions set status = ? where id = ?",
            TransactionStatus::Failed, transaction.id
        }
        .execute(&state.sql)
    };

    if result.is_err() {
        let _ = failed().await;
        return response;
    }

    #[derive(Deserialize, Debug)]
    struct RsData {
        code: i64,
        ref_id: i64,
        card_pan: String,
        card_hash: String,
    }

    let data = result.unwrap().json::<ZarinpalResponse<RsData>>().await;
    if data.is_err() {
        let _ = failed().await;
        return response;
    }
    let data = data.unwrap().data;
    if data.code != 100 {
        log::info!("verify data: {data:#?}");
        let _ = failed().await;
        return response;
    }

    let result = sqlx::query! {
        "update users set wallet = ? where id = ?",
        wallet, user.id
    }
    .execute(&state.sql)
    .await;

    if result.is_err() {
        log::error!(
            "could not update user wallet tid: {} - {}",
            transaction.id,
            transaction.amount
        );
        let _ = failed().await;
        return response;
    }

    let _ = sqlx::query! {
        "update transactions set
        status = ?, ref_id = ?, card = ?, card_hash = ?
        where id = ?",
        TransactionStatus::Success, data.ref_id, data.card_pan, data.card_hash,
        transaction.id
    }
    .execute(&state.sql)
    .await;

    response
}

#[utoipa::path(
    get,
    params(ListInput),
    responses((status = 200, body = Vec<Transaction>))
)]
/// List Transactions
#[get("/transactions/")]
async fn user_transactions(
    user: User, q: Query<ListInput>, state: Data<AppState>,
) -> Response<Vec<Transaction>> {
    let offset = q.page * 32;
    let result = sqlx::query_as! {
        Transaction,
        "select * from transactions where user = ?
         order by id desc limit 32 offset ?",
        user.id, offset
    }
    .fetch_all(&state.sql)
    .await?;

    Ok(Json(result))
}

#[utoipa::path(
    get,
    params(ListInput),
    responses((status = 200, body = Vec<Message>))
)]
/// List Messages
#[get("/messages/")]
async fn user_messages(
    user: User, q: Query<ListInput>, state: Data<AppState>,
) -> Response<Vec<Message>> {
    let offset = q.page * 32;
    let result = sqlx::query_as! {
        Message,
        "select * from messages where user = ? order by id desc limit 32 offset ?",
        user.id, offset
    }
    .fetch_all(&state.sql)
    .await?;

    Ok(Json(result))
}

#[utoipa::path(
    post,
    params(("id" = i64, Path,)),
    responses((status = 200))
)]
/// Message Seen
#[post("/messages/{id}/seen/")]
async fn user_message_seen(
    user: User, message: Message, state: Data<AppState>,
) -> Result<HttpResponse, AppErr> {
    sqlx::query! {
        "update messages set seen = true where id = ? and user = ?",
        message.id, user.id
    }
    .execute(&state.sql)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    get,
    responses((status = 200, body = i32))
)]
/// Messages UnSeen Count
#[get("/messages-unseen-count/")]
async fn user_messages_unseen_count(
    user: User, state: Data<AppState>,
) -> Response<i32> {
    let result = sqlx::query! {
        "select count(id) as count from messages where user = ? and seen = false
        order by id desc limit 10",
        user.id
    }
    .fetch_one(&state.sql)
    .await?;

    Ok(Json(result.count))
}

#[utoipa::path(
    get,
    params(ListInput),
    responses((status = 200, body = Vec<Order>))
)]
/// List Orders
#[get("/orders/")]
async fn user_orders(
    user: User, q: Query<ListInput>, state: Data<AppState>,
) -> Response<Vec<Order>> {
    let offset = q.page * 32;
    let result = sqlx::query_as! {
        Order,
        "select * from orders where user = ? order by id desc limit 32 offset ?",
        user.id, offset
    }
    .fetch_all(&state.sql)
    .await?;

    Ok(Json(result))
}

pub fn router() -> Scope {
    Scope::new("/user")
        .service(user_get)
        .service(user_deposit)
        .service(user_transactions)
        .service(user_messages)
        .service(user_message_seen)
        .service(user_messages_unseen_count)
        .service(user_orders)
}
