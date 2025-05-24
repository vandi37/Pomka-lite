use repository::RepositoryTrait;
use std::sync::Arc;
use teloxide::dispatching::DefaultKey;
use teloxide::prelude::*;
use crate::command::{handle_commands, Command};
use crate::error::Error;
use crate::filter::filter;

mod from_env;
mod macros;
mod models;
mod repository;
mod role;
mod handler;
mod command;
mod filter;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .inspect(|m: Message| {
            let text = m.text().unwrap_or("null").to_string();
            let sender = if let Some(sender) = m.sender_chat {
                let name = sender.first_name().unwrap_or("_");
                format!("sender {} ({})", name, sender.id)
            } else {
                "".to_owned()
            };
            tracing::debug!("Got message '{}' from chat {}, {}", text, m.chat.id, sender);
        }).branch(
        dptree::entry()
            .filter_command::<Command>()
            .endpoint(handle_commands)
    )
        .branch(dptree::entry()
            .endpoint(filter)
        );


        /*.inspect_async(|m: Message| async {
            if let Some(sender) = m.sender_chat {
                let nickname = if let Some(first_name) = sender.first_name() {
                    first_name.to_owned()
                } else if let Some(title) = sender.title() {
                    title.to_owned()
                } else {
                    "_".to_owned()
                };
                if let Err(e) = repo
                    .new_user(
                        sender.id.0,
                        role_selector.select(sender.id.0),
                        sender.username().map(|u: &str| u.to_owned()),
                        nickname,
                    )
                    .await
                {
                    tracing::error!("Failed to create/update user: {:?}", e);
                }
            }
        });*/
    {
        let me = bot.get_me().await.expect("cannot get me");
        tracing::info!("Starting bot {}...", me.username().to_string());
    }

    Dispatcher::<Bot, Error, DefaultKey>::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
