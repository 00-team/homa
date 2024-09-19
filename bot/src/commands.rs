use crate::{config::config, Command, KeyData, StartArgs, State, Store, HR};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

pub async fn handle_commands(
    bot: Bot, store: Store, msg: Message, cmd: Command,
) -> HR {
    let login_button =
        InlineKeyboardButton::login("🪪 ورود", config().login_url.clone());
    let login_keyboard = InlineKeyboardMarkup::new([[login_button.clone()]]);

    match cmd {
        Command::Start(_) => {
            let args = cmd.start_args();
            match args {
                StartArgs::Login => {
                    bot.send_message(msg.chat.id, "ورود به سایت")
                        .reply_markup(login_keyboard)
                        .await?;

                    return Ok(());
                }
                _ => {}
            }

            bot.send_message(
                msg.chat.id,
                indoc::indoc! {"
                    توضیحات درباره تورا
                "},
            )
            .reply_markup(InlineKeyboardMarkup::new([[
                InlineKeyboardButton::callback("منو", KeyData::Menu),
                login_button,
            ]]))
            .await?;

            store.update(State::Menu).await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Menu => {
            store.update(State::Menu).await?;
            bot.send_message(msg.chat.id, "menu...").await?;
        }
        Command::Login => {
            bot.send_message(msg.chat.id, "ورود به سایت")
                .reply_markup(login_keyboard)
                .await?;
        }
    }

    Ok(())
}
