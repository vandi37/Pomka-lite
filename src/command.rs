use teloxide::macros::BotCommands;
use teloxide::prelude::*;
use teloxide::sugar::request::RequestReplyExt;
use teloxide::types::ParseMode;
use crate::error::Error;

#[derive(BotCommands, Clone)]
#[command(rename_rule  = "snake_case")]
pub enum Command {
    Help,
    Start,
}

pub async fn handle_commands(bot: Bot, msg: Message, cmd: Command) -> Result<(), Error> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, "🔰 Вот ссылка на статью по использованию бота > [тык](https://example.com)")
                .reply_to(msg.id)
                .parse_mode(ParseMode::MarkdownV2)
                .send().await?;
        }
        Command::Start => {
           bot.send_message(msg.chat.id, "👋 Привет! Я — Помка, для подробного ознакомления с функционалом напиши /help")
               .reply_to(msg.id)
               .send().await?;

        }
    };
    Ok(())
}
