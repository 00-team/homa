use std::error::Error;
use teloxide::dispatching::dialogue;
use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::utils::command::BotCommands;

pub type Store = dialogue::Dialogue<State, ErasedStorage<State>>;
pub type HR = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyData {
    Unknown,
    Menu,
    ShopStar,
    ChargeWallet,
    BuyStar(u64),
    ComingSoon,
}

impl From<KeyData> for String {
    fn from(value: KeyData) -> Self {
        serde_json::to_string(&value).unwrap()
    }
}

impl From<String> for KeyData {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap_or(KeyData::Unknown)
    }
}

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    #[default]
    Start,
    Menu,
    ShopStar
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
pub enum Command {
    Start(String),
    Menu,
    Help,
    Login,
}

#[derive(Debug, Default)]
pub enum StartArgs {
    #[default]
    None,
    Invite {
        id: i64,
    },
    Login,
}

impl Command {
    pub fn start_args(self) -> StartArgs {
        let args = match self {
            Self::Start(args) => args,
            _ => panic!("only start command has args"),
        };
        let mut tokens = args.split("-");

        let key = if let Some(k) = tokens.next() {
            k
        } else {
            return StartArgs::None;
        };

        match key {
            "login" => StartArgs::Login,
            "invite" => {
                let id = tokens.next().and_then(|v| v.parse::<i64>().ok());
                if let Some(id) = id {
                    StartArgs::Invite { id }
                } else {
                    StartArgs::None
                }
            }
            _ => StartArgs::None,
        }
    }
}
