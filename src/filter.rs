use teloxide::Bot;
use teloxide::prelude::*;
use crate::error::Error;
use crate::repository::RepositoryTrait;
use crate::role::RoleSelector;

pub struct Handler<R>{
    repo: R,
    role_selector: RoleSelector,
}

impl<R> Handler<R>
where R: RepositoryTrait {
    pub fn new(repo: R, role_selector: RoleSelector) -> Self {
        Self { repo, role_selector }
    }

    pub async fn filter(bot: Bot, message: Message) -> Result<(), Error> {
        if let Some(text) = message.text() {
            let (first, after) = text.split_once(char::is_whitespace)
                    .unwrap_or((text, ""));
            match first {
                _ => ()
            };
        }
        Ok(())
    }
}


