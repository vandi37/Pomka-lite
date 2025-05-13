use crate::models::prelude::*;
use sea_orm::sea_query::IntoValueTuple;
use sea_orm::{
    ColumnTrait, EntityTrait, Iterable, PrimaryKeyToColumn, PrimaryKeyTrait, QueryFilter,
    UpdateMany,
};
pub mod db;

pub trait Repository {
    type Error;
    async fn new_user(&self, role: Role) -> Result<i64, Self::Error>;
    async fn block_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn unblock_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn promote_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn demote_user(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn get_user(&self, user: i64) -> Result<UserModel, Self::Error>;

    async fn warn(&self, by: i64, user: i64) -> Result<(), Self::Error>;
    async fn un_warn(&self, by: i64, user: i64) -> Result<(), Self::Error>;

    async fn create_command(&self, command: CommandModel) -> Result<(), Self::Error>;
    async fn update_command(&self, id: String, by: i64, action: String) -> Result<(), Self::Error>;
    async fn delete_command(&self, id: String, by: i64) -> Result<(), Self::Error>;
    async fn get_command(&self, id: String) -> Result<CommandModel, Self::Error>;
    async fn get_user_commands(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<CommandModel>, Self::Error>;
    async fn use_command(&self, id: String, by: i64) -> Result<(), Self::Error>;

    async fn new_action(&self, action: ActionModel) -> Result<(), Self::Error>;
    async fn get_action(&self, id: i64) -> Result<ActionModel, Self::Error>;
    async fn get_user_actions(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<ActionModel>, Self::Error>;
}
