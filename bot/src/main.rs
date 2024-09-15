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
mod commands;
mod callback;

use config::config;
use types::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_path(".env").expect("no .env");
    pretty_env_logger::init();

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
                        .endpoint(commands::handle_commands),
                )
            ,
        )
        .branch(
            Update::filter_callback_query()
                .enter_dialogue::<Update, ErasedStorage<State>, State>()
                .endpoint(callback::callback_query),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .build()
        .dispatch()
        .await;

    Ok(())
}

