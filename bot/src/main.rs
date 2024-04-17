use sqlx::SqlitePool;
use std::env;
use std::error::Error;
use std::sync::Arc;
use teloxide::dispatching::dialogue;
use teloxide::dispatching::dialogue::serializer::Json;
use teloxide::dispatching::dialogue::{ErasedStorage, SqliteStorage, Storage};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::prelude::*;
// use teloxide::types::ParseMode::MarkdownV2;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
};
use teloxide::utils::command::BotCommands;

mod config;
mod tools;

use config::config;

type Dialogue = dialogue::Dialogue<State, ErasedStorage<State>>;
type HR = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum KeyData {
    Tutorial,
    Buy,
    Rent,
}

impl From<KeyData> for String {
    fn from(value: KeyData) -> Self {
        serde_json::to_string(&value).unwrap()
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum PurchaseKind {
    #[default]
    Buy,
    Rent,
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    Tutorial,
    SelectService {
        purchase_kind: PurchaseKind,
    },
    SelectCountry {
        purchase_kind: PurchaseKind,
        service: i64,
    },
    Confirm {
        purchase_kind: PurchaseKind,
        service: i64,
        country: i64,
    },
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
pub enum Command {
    Start(String),
    Help,
    /// user info
    MyInfo,
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
    bot: Bot, dlg: Dialogue, pool: &SqlitePool, msg: Message, cmd: Command,
) -> HR {
    match cmd {
        Command::Start(arg) => {
            let arg = tools::parse_start_args(&arg);

            bot.send_message(msg.chat.id, KeyData::Rent).await?;

            let inline = [
                [InlineKeyboardButton::callback("Tutorial 📖", "tutorial")],
                [InlineKeyboardButton::callback("Buy Virtual Number", "buy")],
                [InlineKeyboardButton::callback("Rent Virtual Number", "rent")],
            ];
            let keyboard =
                [[KeyboardButton::new("Hi")], [KeyboardButton::new("2")]];

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
    }

    Ok(())
}

async fn cbq(bot: Bot, dlg: Dialogue, q: CallbackQuery) -> HR {
    bot.answer_callback_query(q.id).await?;
    if q.message.is_none() || q.data.is_none() {
        return Ok(());
    }

    let msg = q.message.unwrap();
    let data = q.data.unwrap();
    let key: KeyData = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => return Ok(()),
    };

    bot.send_message(msg.chat.id, format!("key: {:?}", key)).await?;

    Ok(())
}
