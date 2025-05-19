use crate::repository::db::RepoError;
use crate::repository::RepositoryTrait;
use teloxide::prelude::*;
use teloxide::types::MessageKind::Common;
use teloxide::types::{MediaKind, MediaText, MessageCommon, MessageEntityKind};
use crate::models::prelude::UserModel;

impl<R> crate::handler::Handler<R>
where
    R: RepositoryTrait<Error = RepoError>,
{
    pub async fn get_user(&self, message: &mut Message) -> Result<UserModel, RepoError> {
        if let Some(from) = if let Some(reply) = message.reply_to_message() {
            &reply.from
        } else {
            &None
        } {
            self.repo.get_user(from.id.0 as i64).await
        } else if let Some(entities) = message.entities() {
            for entity in entities {
                match &entity.kind {
                    MessageEntityKind::Mention => {
                        let text = message.text().unwrap_or_default();
                        let mention = &text[entity.offset..entity.offset + entity.length];
                        let mention_without_at = mention.trim_start_matches('@');
                        let mut new_text = text.to_string();
                        new_text.replace_range(entity.offset..entity.offset + entity.length, "");
                        let result = self.repo.get_user_by_username(mention_without_at.to_owned()).await;
                        set_message_text(message, new_text);
                        return result
                    }
                    MessageEntityKind::TextMention { user } => {
                        let text = message.text().unwrap_or_default();
                        let mut new_text = text.to_string();
                        new_text.replace_range(entity.offset..entity.offset + entity.length, "");
                        let result = self.repo.get_user(user.id.0 as i64).await;
                        set_message_text(message, new_text);
                        return result;
                    }
                    _ => continue,
                };
            };
            Err(RepoError::NotFound)
        } else {
            Err(RepoError::NotFound)
        }
    }
}


fn set_message_text(message: &mut Message, new_text: String) {
    if let Common(MessageCommon {
                      media_kind: MediaKind::Text(MediaText { text, .. }),
                      ..
                  }) = &mut message.kind
    {
        *text = new_text
    }
}

