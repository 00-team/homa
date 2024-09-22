use indoc::formatdoc;
use teloxide::prelude::*;
use teloxide::types::InlineKeyboardButton;
use teloxide::types::InlineKeyboardMarkup;
use teloxide::types::User;

use crate::config::config;
use crate::types::*;
use crate::{api, utils};

pub async fn menu_send(bot: &Bot, store: Store, user: User) -> HR {
    let config = config();
    if let Some(tu) = api::user_get(user.id.0).await? {
        let star_price = api::star_price(user.id.0).await?;
        bot.send_message(
            user.id,
            formatdoc! {"
                نام: {}
                کیف پول: {} تومان
                استار: {} ⭐

                --- thora ---
            ",
                user.full_name(), utils::toman(tu.wallet),
                (tu.wallet as f64 / star_price) as i64
            },
        )
        .reply_markup(InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::login(
                "ورود مججد",
                config.login_url.clone(),
            )],
            vec![
                InlineKeyboardButton::url(
                    "سفارشات 🛍",
                    config.orders_url.clone(),
                ),
                InlineKeyboardButton::callback(
                    "شارژ کیف پول 💰",
                    KeyData::ChargeWallet,
                ),
            ],
            vec![
                InlineKeyboardButton::callback(
                    "خرید استارز ⭐",
                    KeyData::ShopStar,
                ),
                InlineKeyboardButton::callback(
                    "خرید شماره مجازی 📞",
                    KeyData::ComingSoon,
                ),
            ],
            vec![InlineKeyboardButton::callback(
                "استار رایگان 💰",
                KeyData::ComingSoon,
            )],
        ]))
        .await?;
    } else {
        register(bot, &store, user.id).await?;
    }

    store.update(State::Menu).await?;

    Ok(())
}

pub async fn register(bot: &Bot, store: &Store, user_id: UserId) -> HR {
    bot.send_message(user_id, "ثبت نام در وب سایت تورا الزامی می باشد")
        .reply_markup(InlineKeyboardMarkup::new([[
            InlineKeyboardButton::login("ثبت نام", config().login_url.clone()),
        ]]))
        .await?;

    store.update(State::Register).await?;

    Ok(())
}
