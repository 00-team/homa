async fn handle_commands(
    bot: Bot, store: Store, msg: Message, cmd: Command,
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
                // .reply_markup(KeyboardMarkup::new(keyboard))
                .await?;

            bot.send_message(msg.chat.id, "متن استارت")
                // .reply_markup(InlineKeyboardMarkup::new(inline))
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
