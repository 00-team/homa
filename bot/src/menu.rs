use indoc::formatdoc;
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardButton;
use teloxide::types::InlineKeyboardMarkup;
use teloxide::types::User;

use crate::config::config;
use crate::types::*;
use crate::{api, utils};

pub async fn menu_send(bot: Bot, store: Store, user: User) -> HR {
    if let Some(tu) = api::user_get(user.id.0).await? {
        bot.send_message(
            user.id,
            formatdoc! {"
                نام: {}
                کیف پول: {:} تومان
            ", user.full_name(), utils::toman(tu.wallet)},
        )
        .reply_markup(InlineKeyboardMarkup::new([[
            InlineKeyboardButton::login(
                "update info",
                config().login_url.clone(),
            ),
        ]]))
        .await?;
    } else {
        bot.send_message(user.id, "please register")
            .reply_markup(InlineKeyboardMarkup::new([[
                InlineKeyboardButton::login(
                    "register",
                    config().login_url.clone(),
                ),
            ]]))
            .await?;
    }

    store.update(State::Menu).await?;

    Ok(())
}
