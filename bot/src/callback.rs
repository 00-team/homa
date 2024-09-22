use crate::{
    api, config::Config, menu::menu_send, utils::toman, KeyData, State, Store,
    HR,
};
use indoc::formatdoc;
use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

pub async fn callback_query(bot: Bot, store: Store, q: CallbackQuery) -> HR {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().clone();
    if q.message.is_none() || q.data.is_none() || chat_id.is_none() {
        return Ok(());
    }

    let (msg, chat_id) = (q.message.unwrap(), chat_id.unwrap());
    let data = q.data.unwrap();
    let key: KeyData = data.into();

    match key {
        KeyData::Menu => {
            menu_send(&bot, store, q.from).await?;
            bot.delete_message(chat_id, msg.id()).await?;
            return Ok(());
        }
        KeyData::ChargeWallet => {
            let user = api::user_get(q.from.id.0).await?;
            if user.is_none() {
                bot.send_message(chat_id, "register first").await?;
                store.update(State::Menu).await?;
                return Ok(());
            }
            let user = user.unwrap();
            let mut keyboard = [50, 100, 250, 500, 1700]
                .iter()
                .map(|a| *a * 1_000_0)
                .filter(|irr| user.wallet + *irr < 5_000_000_0)
                .map(|irr| {
                    [InlineKeyboardButton::url(
                        format!("{} تومان", toman(irr)),
                        Config::wallet(irr),
                    )]
                })
                .collect::<Vec<_>>();
            keyboard.insert(
                0,
                [InlineKeyboardButton::callback("منو 📜", KeyData::Menu)],
            );

            bot.send_message(chat_id, "شارژ کیف پول 💰")
                .reply_markup(InlineKeyboardMarkup::new(keyboard))
                .await?;

            bot.delete_message(chat_id, msg.id()).await?;
            return Ok(());
        }
        _ => {}
    }

    let state = store.get_or_default().await?;
    match state {
        State::Menu => match key {
            KeyData::ComingSoon => {
                bot.send_message(chat_id, "به زودی 🌊").await?;
            }
            KeyData::ShopStar => {
                let user = api::user_get(q.from.id.0).await?;
                if user.is_none() {
                    bot.send_message(chat_id, "register first").await?;
                    store.update(State::Menu).await?;
                    return Ok(());
                }
                let user = user.unwrap();
                let star_price = api::star_price(q.from.id.0).await?;
                let mut keyboard = [50, 75, 100, 150, 250, 350, 2500, 4000]
                    .iter()
                    .filter(|a| **a as f64 * star_price < user.wallet as f64)
                    .map(|a| {
                        [InlineKeyboardButton::callback(
                            format!(
                                "⭐ {a} - {} تومان",
                                toman((*a as f64 * star_price) as i64)
                            ),
                            KeyData::BuyStar(*a),
                        )]
                    })
                    .collect::<Vec<_>>();
                keyboard.insert(
                    0,
                    [InlineKeyboardButton::callback("منو 📜", KeyData::Menu)],
                );
                keyboard.insert(
                    1,
                    [InlineKeyboardButton::callback(
                        "شارژ کیف پول 💰",
                        KeyData::ChargeWallet,
                    )],
                );
                bot.send_message(
                    chat_id,
                    formatdoc! {"
                        خرید استار تلگرام ⭐
                        کیف پول: {} تومان
                        استارز: {} ⭐

                        --- thora ---
                    ",
                        toman(user.wallet),
                        (user.wallet as f64 / star_price) as i64
                    },
                )
                .reply_markup(InlineKeyboardMarkup::new(keyboard))
                .await?;
                store.update(State::ShopStar).await?;
                bot.delete_message(chat_id, msg.id()).await?;
            }
            _ => {}
        },
        State::ShopStar => match key {
            KeyData::BuyStar(amount) => {
                let result = api::stars_buy(q.from.id.0, amount).await?;
                let dpy = match result {
                    Ok(v) => String::from(v),
                    Err(e) => String::from(e),
                };

                bot.send_message(chat_id, dpy)
                    .reply_markup(InlineKeyboardMarkup::new([[
                        InlineKeyboardButton::callback("منو 📜", KeyData::Menu),
                    ]]))
                    .await?;
                bot.delete_message(chat_id, msg.id()).await?;
            }
            _ => {}
        },
        _ => (),
    }

    Ok(())
}
