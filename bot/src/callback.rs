use crate::{KeyData, State, Store, HR};
use teloxide::{dispatching::dialogue::GetChatId, prelude::*};

pub async fn callback_query(bot: Bot, store: Store, q: CallbackQuery) -> HR {
    bot.answer_callback_query(q.id).await?;
    let chat_id = q.chat_id();
    if q.message.is_none() || q.data.is_none() || chat_id.is_none() {
        return Ok(());
    }

    let (msg, chat_id) = (q.message.unwrap(), chat_id.unwrap());
    let data = q.data.unwrap();
    let key: KeyData = data.into();

    let state = store.get_or_default().await?;
    match state {
        State::Start => match key {
            KeyData::Menu => {
                bot.send_message(chat_id, "menu").await?;
                store.update(State::Menu).await?;
            }
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
