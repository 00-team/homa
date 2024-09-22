use crate::{
    config::{config, Config},
    models::{
        order::{OrderStatus, StarOrder},
        user::User,
        AppErr,
    },
};
use actix_web::web::Buf;
use indoc::formatdoc;
use rand::Rng;
use serde::Serialize;
use std::{fs::File, io};

pub fn now() -> i64 {
    chrono::Local::now().timestamp()
}

pub fn get_random_string(charset: &[u8], len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| charset[rng.gen_range(0..charset.len())] as char).collect()
}

// pub fn get_random_bytes(len: usize) -> String {
//     let mut rng = rand::thread_rng();
//     hex::encode((0..len).map(|_| rng.gen::<u8>()).collect::<Vec<u8>>())
// }

pub async fn save_photo(url: &str, id: i64) -> Result<(), AppErr> {
    let client = awc::Client::new();
    let mut result = client.get(url).send().await?.body().await?.reader();
    let mut file = File::create(format!("{}/u-{id}.jpg", Config::RECORD_DIR))?;

    io::copy(&mut result, &mut file)?;

    Ok(())
}

pub async fn send_message(chat_id: i64, text: &str) {
    let client = awc::Client::new();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        config().bot_token
    );
    let request = client.post(&url);

    #[derive(Serialize, Debug)]
    struct Body {
        chat_id: i64,
        text: String,
    }

    let _ = request.send_json(&Body { chat_id, text: text.to_string() }).await;
}

pub async fn send_after_login(chat_id: i64) {
    let client = awc::Client::new();
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        config().bot_token
    );
    let request = client.post(&url);

    #[derive(Serialize, Debug)]
    struct Btn {
        text: &'static str,
        callback_data: &'static str,
    }

    #[derive(Serialize, Debug)]
    struct Markup {
        inline_keyboard: [[Btn; 1]; 1],
    }

    #[derive(Serialize, Debug)]
    struct Body {
        chat_id: i64,
        text: &'static str,
        reply_markup: Markup,
    }

    let _ = request
        .send_json(&Body {
            chat_id,
            reply_markup: Markup {
                inline_keyboard: [[Btn {
                    text: "منو 📜",
                    callback_data: "\"menu\"",
                }]],
            },
            text: "به تورا خوش آمدید 🎉",
        })
        .await;
}

pub fn toman(irr: i64) -> String {
    (irr / 10)
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .expect("utils::toman failed")
        .join(",")
}

pub async fn send_star_order(user: &User, order: &StarOrder) {
    let config = config();
    let client = awc::Client::new();
    let url =
        format!("https://api.telegram.org/bot{}/sendMessage", config.bot_token);
    let request = client.post(&url);

    #[derive(Serialize, Debug)]
    struct Btn {
        text: &'static str,
        url: String,
    }

    #[derive(Serialize, Debug)]
    struct Markup {
        inline_keyboard: [[Btn; 1]; 1],
    }

    #[derive(Serialize, Debug)]
    struct Body {
        chat_id: i64,
        text: String,
        reply_markup: Markup,
    }

    let status = match order.status {
        OrderStatus::Done => "تکمیل شده ✅",
        OrderStatus::Wating => "درحال تکمیل ⏳",
        OrderStatus::Refunded => "ریفاند شده ❌",
    };

    let _ = request
        .send_json(&Body {
            chat_id: config.trust,
            reply_markup: Markup {
                inline_keyboard: [[Btn {
                    text: "خرید",
                    url: config.bot_url.clone(),
                }]],
            },
            text: formatdoc! {"
                سفارش استار ⭐
                وضعیت سفارش: {}
                تعداد: {} ⭐
                قیمت: {} تومان
                خریداد: {}

                --- thora ---
            ",
                status, order.amount, toman(order.cost), user.name
            },
        })
        .await;
}

pub trait CutOff {
    fn cut_off(&mut self, len: usize);
}

impl CutOff for String {
    fn cut_off(&mut self, len: usize) {
        let mut idx = len;
        loop {
            if self.is_char_boundary(idx) {
                break;
            }
            idx -= 1;
        }
        self.truncate(idx)
    }
}

impl CutOff for Option<String> {
    fn cut_off(&mut self, len: usize) {
        if let Some(v) = self {
            let mut idx = len;
            loop {
                if v.is_char_boundary(idx) {
                    break;
                }
                idx -= 1;
            }
            v.truncate(idx)
        }
    }
}
