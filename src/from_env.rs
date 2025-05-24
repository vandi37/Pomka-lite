use crate::repository::db::{Repository, RepositoryOptions};
use crate::repository::RepositoryTrait;
use crate::role::RoleSelector;
use sea_orm::prelude::*;
use sea_orm::Database;
use std::env::VarError;

const DATABASE_URL: &'static str = "DATABASE_URL";
const MAX_WARNS: &'static str = "MAX_WARNS";
const CREATOR: &'static str = "CREATOR";

#[derive(Debug)]
pub enum ConnError {
    VarError(VarError),
    DbError(DbErr),
}

impl From<VarError> for ConnError {
    fn from(e: VarError) -> Self {
        ConnError::VarError(e)
    }
}
impl From<DbErr> for ConnError {
    fn from(e: DbErr) -> Self {
        ConnError::DbError(e)
    }
}
pub async fn connection_from_env() -> Result<DatabaseConnection, ConnError> {
    Ok(Database::connect(&std::env::var(DATABASE_URL)?).await?)
}

pub fn max_warns_from_env() -> Option<i64> {
    std::env::var(MAX_WARNS).ok().and_then(|s| s.parse().ok())
}

pub async fn repo_from_env() -> Result<Repository, ConnError> {
    Ok(Repository::new(RepositoryOptions {
        database: connection_from_env().await?,
        max_warns: max_warns_from_env(),
    }))
}

pub fn role_selector_from_env() -> RoleSelector {
    RoleSelector::new(std::env::var(CREATOR).ok().and_then(|c| c.parse().ok()))
}
