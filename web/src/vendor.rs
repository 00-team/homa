use std::{collections::HashSet, env, str::FromStr};

use lazy_static::lazy_static;
use serde_json::{json, Number, Value};

use crate::models::AppErr;

lazy_static! {
    static ref BASE_URL: String = {
        format!(
            "https://api.sms-activate.org/stubs/handler_api.php?api_key={}",
            env::var("VENDOR_APIKEY").unwrap()
        )
    };
    static ref ERRORS: HashSet<&'static str> = {
        HashSet::from([
            "NO_NUMBERS",
            "NO_BALANCE",
            "BAD_ACTION",
            "BAD_SERVICE",
            "BAD_KEY",
            "ERROR_SQL",
            "SQL_ERROR",
            "NO_ACTIVATION",
            "BAD_STATUS",
            "STATUS_CANCEL",
            "BANNED",
            "NO_CONNECTION",
            "ACCOUNT_INACTIVE",
            "NO_ID_RENT",
            "INVALID_PHONE",
            "STATUS_FINISH",
            "INCORECT_STATUS",
            "CANT_CANCEL",
            "ALREADY_FINISH",
            "ALREADY_CANCEL",
            "WRONG_OPERATOR",
            "NO_YULA_MAIL",
            "WHATSAPP_NOT_AVAILABLE",
            "NOT_INCOMING",
            "INVALID_ACTIVATION_ID",
            "WRONG_ADDITIONAL_SERVICE",
            "WRONG_ACTIVATION_ID",
            "WRONG_SECURITY",
            "REPEAT_ADDITIONAL_SERVICE",
            "NO_KEY",
            "OPERATORS_NOT_FOUND",
        ])
    };
}

pub async fn request(
    action: &'static str, args: Vec<(&'static str, &str)>,
) -> Result<Value, AppErr> {
    let p = args.iter().map(|(a, b)| format!("&{a}={b}")).collect::<String>();
    let url = format!("{}&action={}{}", *BASE_URL, action, p);

    let response = String::from_utf8(
        awc::Client::new().get(url).send().await?.body().await?.to_vec(),
    )?;

    if response.len() <= 25 && ERRORS.contains(response.as_str()) {
        log::error!("response: {response}");
        return Err(AppErr::new(500, "service not available"));
    }

    match action {
        "getBalance" | "getBalanceAndCashBack" => {
            Ok(Value::Number(Number::from_str(response.split_at(15).1)?))
        }
        "getNumber" => {
            let mut result = response.split_at(14).1.splitn(2, ':');
            let id = Number::from_str(result.next().unwrap())?;
            let phone = Number::from_str(result.next().unwrap())?;
            Ok(json!({ "id": id, "phone": phone }))
        }
        "getAdditionalService" => {
            let mut result = response.split_at(11).1.splitn(2, ':');
            let id = Number::from_str(result.next().unwrap())?;
            let phone = Number::from_str(result.next().unwrap())?;
            Ok(json!({ "id": id, "phone": phone }))
        }
        _ => {
            Ok(serde_json::from_str::<Value>(&response)?)
        }
    }
}
