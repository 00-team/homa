use indoc::{formatdoc, indoc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest;
use sqlx::SqlitePool;
use std::env;
use std::error::Error;
use std::sync::Arc;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::{self, GetChatId};
use teloxide::dispatching::dialogue::{ErasedStorage, SqliteStorage, Storage};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::prelude::*;
// use teloxide::types::ParseMode::MarkdownV2;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup};
use teloxide::utils::command::BotCommands;

mod config;
mod tools;

use config::config;

type Dialogue = dialogue::Dialogue<State, ErasedStorage<State>>;
type HR = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Menu,
    AddRecord {
        id: i64,
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
pub enum Command {
    Start(String),
    Help,
    // /// make a new record
    // NewRecord,
    // /// get a record by id
    // GetRecord {
    //     id: i64,
    // },
    // /// list of all records
    // ListRecord,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
pub enum RecordCommand {
    /// finish sending messages for records
    EndRecord,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_path("../.secrets.env").unwrap();
    pretty_env_logger::init();

    let pool: &'static SqlitePool = Box::leak(Box::new(
        SqlitePool::connect(&env::var("DATABASE_URL")?).await?,
    ));
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
        .branch(Update::filter_callback_query().endpoint(cbq));

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
    bot: Bot,
    dlg: Dialogue,
    pool: &SqlitePool,
    msg: Message,
    cmd: Command,
) -> HR {
    match cmd {
        Command::Start(arg) => {
            let arg = tools::parse_start_args(&arg);
            let inline = [
                [InlineKeyboardButton::callback("Tutorial 📖", "tutorial")],
                [InlineKeyboardButton::callback("Buy Virtual Number", "buy")],
                [InlineKeyboardButton::callback(
                    "Rent Virtual Number",
                    "rent",
                )],
            ];
            let keyboard = [[KeyboardButton::new("Hi")], [KeyboardButton::new("2")]];

            bot.send_message(msg.chat.id, "متن استارت")
                .reply_markup(InlineKeyboardMarkup::new(inline))
                .reply_markup(KeyboardMarkup::new(keyboard))
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
        } // Command::NewRecord => new_record(bot, dlg, pool, msg).await?,
          // Command::GetRecord { id } => get_record(bot, pool, id, msg).await?,
          // Command::ListRecord => list_record(bot, pool, msg).await?,
    }

    Ok(())
}

async fn cbq(bot: Bot, q: CallbackQuery) -> HR {
    bot.answer_callback_query(q.id.clone()).await?;
    if let Some(msg) = &q.message {
        bot.send_message(msg.chat.id, format!("{q:#?}")).await?;
        bot.send_message(msg.chat.id, "hi this is gg").await?;
    }

    Ok(())
}
