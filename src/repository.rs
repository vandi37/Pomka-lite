use crate::models::actions::Type;
use crate::models::prelude::*;
use sea_orm::prelude::Json;
pub mod db;

pub trait RepositoryTrait {
    type Error;
    type Options;
    fn new(options: Self::Options) -> Self;
    async fn new_user(
        &self,
        id: i64,
        role: Role,
        username: Option<String>,
        nickname: String,
    ) -> Result<(), Self::Error>;
    async fn change_nickname(&self, by: i64, id: i64, nickname: String) -> Result<(), Self::Error>;
    async fn block_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn unblock_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn promote_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn demote_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn get_user(&self, user: i64) -> Result<UserModel, Self::Error>;
    async fn get_user_by_username(&self, username: String) -> Result<UserModel, Self::Error>;

    async fn warn(&self, by: i64, user: i64) -> Result<bool, Self::Error>;
    async fn un_warn(&self, by: i64, user: i64) -> Result<(), Self::Error>;

    async fn create_command(
        &self,
        name: String,
        action: String,
        creator: i64,
    ) -> Result<(), Self::Error>;
    async fn update_command(&self, id: String, by: i64, action: String) -> Result<(), Self::Error>;
    async fn delete_command(&self, id: String, by: i64) -> Result<(), Self::Error>;
    async fn get_command(&self, id: String) -> Result<CommandModel, Self::Error>;
    async fn get_user_commands(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<CommandModel>, Self::Error>;
    async fn get_commands(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<CommandModel>, Self::Error>;
    async fn use_command(&self, id: String, by: i64) -> Result<(), Self::Error>;

    async fn new_action(
        &self,
        user_id: i64,
        action_type: Type,
        description: Json,
    ) -> Result<i64, Self::Error>;
    async fn get_action(&self, id: i64) -> Result<ActionModel, Self::Error>;
    async fn get_user_actions(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<ActionModel>, Self::Error>;
    async fn get_actions(&self, page: u64, page_size: u64)
        -> Result<Vec<ActionModel>, Self::Error>;
}
