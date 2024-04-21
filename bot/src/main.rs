use reqwest::Url;
use sqlx::SqlitePool;
use std::env;
use std::sync::Arc;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::{ErasedStorage, SqliteStorage, Storage};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::*;
use tools::StartArg;
// use teloxide::types::ParseMode::MarkdownV2;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
    LoginUrl,
};
use teloxide::utils::command::BotCommands;

mod config;
mod state;
mod tools;
mod types;

use config::config;
use types::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_path("../.secrets.env").unwrap();
    pretty_env_logger::init();

    let pool: &'static SqlitePool =
        Box::leak(Box::new(SqlitePool::connect("sqlite://main.db").await?));
    sqlx::migrate!().run(pool).await?;

    let bot = Bot::from_env();

    bot.send_message(config().dev, "Starting 🐧").await?;

    let storage: Arc<ErasedStorage<State>> =
        SqliteStorage::open(&env::var("TELOXIDE_STORAGE")?, Json)
            .await?
            .erase();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, ErasedStorage<State>, State>()
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(handle_commands),
                ),
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<Update, ErasedStorage<State>, State>()
                .endpoint(cbq),
        );

    // .branch(dptree::case![State::Menu].endpoint(menu))
    // .branch(
    //     dptree::case![State::AddRecord { id }]
    //         .branch(
    //             dptree::entry()
    //                 .filter_command::<RecordCommand>()
    //                 .endpoint(record_commands),
    //         )
    //         .endpoint(add_record),
    // );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage, pool])
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn handle_commands(
    bot: Bot, chit: ChitChat, pool: &SqlitePool, msg: Message, cmd: Command,
) -> HR {
    let login_keyboard =
        InlineKeyboardMarkup::new([[InlineKeyboardButton::login(
            "🪪 Login",
            LoginUrl {
                url: Url::parse(
                    "https://thora.dozar.bid/api/auth/login-telegram/",
                )?,
                forward_text: Some("some forward text".to_string()),
                bot_username: None,
                request_write_access: Some(true),
            },
        )]]);

    match cmd {
        Command::Start(arg) => {
            let arg = tools::parse_start_args(&arg);
            match arg {
                StartArg::Login => {
                    bot.send_message(msg.chat.id, "ورود به سایت")
                        .reply_markup(login_keyboard)
                        .await?;

                    return Ok(());
                }
                _ => {}
            }

            bot.send_message(msg.chat.id, KeyData::Rent).await?;

            let inline = [
                [InlineKeyboardButton::callback(
                    "Tutorial 📖",
                    KeyData::Tutorial,
                )],
                [InlineKeyboardButton::callback("Buy 💰", KeyData::Buy)],
                [InlineKeyboardButton::callback("Rent 💳", KeyData::Rent)],
            ];
            let keyboard = [
                [KeyboardButton::new("Buy 💰"), KeyboardButton::new("Rent 💳")],
                [KeyboardButton::new("My Info 👤"), KeyboardButton::new("Hi")],
            ];

            bot.send_message(msg.chat.id, "Welcome")
                .reply_markup(KeyboardMarkup::new(keyboard))
                .await?;

            bot.send_message(msg.chat.id, "متن استارت")
                .reply_markup(InlineKeyboardMarkup::new(inline))
                .await?;

            // let arg = parse_start_args(&arg);
            // match arg {
            //     StartArg::Record { id, slug: _ } => {
            //         get_record(bot, pool, id, msg).await?;
            //     }
            //     StartArg::None => {
            //         bot.send_message(msg.chat.id, "Welcome to the Neptun Bot.")
            //             .await?;
            //     }
            // }
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::MyInfo => {
            bot.send_message(msg.chat.id, format!("user info for {:#?}", msg))
                .await?;
        }
        Command::Login => {
            bot.send_message(msg.chat.id, "Login to the site and buy")
                .reply_markup(login_keyboard)
                .await?;
        }
    }

    Ok(())
}

async fn cbq(
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
