use indoc::{formatdoc, indoc};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use reqwest;
use sqlx::SqlitePool;
use std::env;
use std::error::Error;
use std::sync::Arc;
use teloxide::dispatching::dialogue;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::{ErasedStorage, SqliteStorage, Storage};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::prelude::*;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommands;

mod config;
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

    let handler = Update::filter_message()
        .enter_dialogue::<Message, ErasedStorage<State>, State>()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(handle_commands),
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
    bot: Bot,
    dlg: Dialogue,
    pool: &SqlitePool,
    msg: Message,
    cmd: Command,
) -> HR {
    match cmd {
        Command::Start(arg) => {
bot.send_message(msg.chat.id, "hi")
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
        },
        // Command::NewRecord => new_record(bot, dlg, pool, msg).await?,
        // Command::GetRecord { id } => get_record(bot, pool, id, msg).await?,
        // Command::ListRecord => list_record(bot, pool, msg).await?,
    }

    Ok(())
}

#[derive(Debug)]
enum StartArg {
    Record { id: i64, slug: String },
    None,
}

fn parse_start_args(arg: &str) -> StartArg {
    let mut value = arg.split("-");
    match value.nth(0) {
        None => StartArg::None,
        Some(key) => match key {
            "record" => {
                if let Some(id) = value.nth(0) {
                    if let Ok(id) = id.parse::<i64>() {
                        if let Some(slug) = value.nth(0) {
                            return StartArg::Record {
                                id,
                                slug: slug.to_owned(),
                            };
                        }
                    }
                }

                StartArg::None
            }
            _ => StartArg::None,
        },
    }
}
