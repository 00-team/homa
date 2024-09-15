async fn callback_query(
    bot: Bot, chit: ChitChat, pool: &SqlitePool, q: CallbackQuery,
) -> HR {
    bot.answer_callback_query(q.id).await?;
    if q.message.is_none() || q.data.is_none() {
        return Ok(());
    }

    let msg = q.message.unwrap();
    let data = q.data.unwrap();
    let key: KeyData = data.into();

    let state = chit.get_or_default().await?;
    match state {
        State::Start => match key {
            KeyData::Buy => {
                chit.update(State::SelectService {
                    purchase_kind: PurchaseKind::Buy,
                })
                .await?
            }
            KeyData::Rent => {
                chit.update(State::SelectService {
                    purchase_kind: PurchaseKind::Rent,
                })
                .await?
            }
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
