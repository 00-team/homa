use std::error::Error;
use teloxide::dispatching::dialogue;
use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::utils::command::BotCommands;

pub type ChitChat = dialogue::Dialogue<State, ErasedStorage<State>>;
pub type HR = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum KeyData {
    Unknown,
    Tutorial,
    Buy,
    Rent,
    Country(i64),
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
