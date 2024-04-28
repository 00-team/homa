use std::{collections::HashSet, env};

use lazy_static::lazy_static;

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
) -> Result<(), AppErr> {
    let p = args.iter().map(|(a, b)| format!("&{a}={b}")).collect::<String>();
    let url = format!("{}&action={}{}", *BASE_URL, action, p);

    let response = awc::Client::new().get(url).send().await?;
    log::warn!("{:#?}", response);

    Ok(())
}
