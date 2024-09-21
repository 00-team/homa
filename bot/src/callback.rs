use crate::{menu::menu_send, KeyData, State, Store, HR};
use teloxide::{
    dispatching::dialogue::GetChatId,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
};

pub async fn callback_query(bot: Bot, store: Store, q: CallbackQuery) -> HR {
    bot.answer_callback_query(&q.id).await?;
    let chat_id = q.chat_id().clone();
    if q.message.is_none() || q.data.is_none() || chat_id.is_none() {
        return Ok(());
    }

    let (_msg, chat_id) = (q.message.unwrap(), chat_id.unwrap());
    let data = q.data.unwrap();
    let key: KeyData = data.into();

    if matches!(key, KeyData::Menu) {
        menu_send(bot, store, q.from).await?;
        return Ok(());
    }

    let state = store.get_or_default().await?;
    match state {
        State::Menu => match key {
            KeyData::ComingSoon => {
                bot.send_message(chat_id, "به زودی 🌊").await?;
            }
            KeyData::ShopStar => {
                bot.send_message(chat_id, "stars ...")
                    .reply_markup(InlineKeyboardMarkup::new(
                        [50, 75, 100, 120, 69].map(|a| {
                            [InlineKeyboardButton::callback(
                                format!("{a} stars ⭐"),
                                KeyData::BuyStar(a),
                            )]
                        }),
                    ))
                    .await?;
                store.update(State::ShopStar).await?;
            }
            _ => {}
        },
        State::Start => match key {
            // KeyData::Buy => {
            //     store
            //         .update(State::SelectService {
            //             purchase_kind: PurchaseKind::Buy,
            //         })
            //         .await?
            // }
            // KeyData::Rent => {
            //     store
            //         .update(State::SelectService {
            //             purchase_kind: PurchaseKind::Rent,
            //         })
            //         .await?
            // }
            _ => (),
        },
        _ => (),
    }

    // match key {
    //     KeyData::Rent => match state {
    //         State::Start => {
    //             dlg.update(State::SelectService {
    //                 purchase_kind: PurchaseKind::Rent,
    //             })
    //             .await?
    //         }
    //         _ => (),
    //     },
    //     KeyData::Buy => {}
    //     KeyData::Country(id) => {}
    //     _ => {}
    // }
    //
    // bot.send_message(msg.chat.id, format!("key: {:?}", key)).await?;

    Ok(())
}
