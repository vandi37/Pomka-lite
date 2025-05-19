use crate::models::actions::Type;
use crate::models::{actions, commands, prelude::*, users};
use crate::{action, error, models, update};
use sea_orm::prelude::Json;
use sea_orm::{
    sqlx, ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QuerySelect, RuntimeErr,
};
use sea_orm_migration::sea_query::Expr;
use serde_json::json;

pub struct Repository {
    db: DatabaseConnection,
    max_warns: i64,
}

impl Repository {
    const DEFAULT_MAX_WARNS: i64 = 5;
}

#[derive(Default)]
pub struct RepositoryOptions {
    pub database: DatabaseConnection,
    pub max_warns: Option<i64>,
}

impl super::RepositoryTrait for Repository {
    type Error = RepoError;
    type Options = RepositoryOptions;
    fn new(options: Self::Options) -> Self {
        Self {
            db: options.database,
            max_warns: options.max_warns.unwrap_or(Self::DEFAULT_MAX_WARNS),
        }
    }

    async fn new_user(
        &self,
        id: i64,
        role: Role,
        username: Option<String>,
        nickname: String,
    ) -> Result<(), Self::Error> {
        if UserEntity::find_by_id(id).count(&self.db).await? > 0 {
            update!(UserEntity: id => {
                Username: username,
            })
            .exec(&self.db)
            .await?;
        } else {
            User {
                id: Set(id),
                username: Set(username),
                nickname: Set(nickname),
                role: Set(role),
                ..Default::default()
            }
            .insert(&self.db)
            .await?;
            action!(self; CreateUser@id => json!({}));
        }
        Ok(())
    }

    async fn change_nickname(&self, by: i64, id: i64, nickname: String) -> Result<(), Self::Error> {
        if by != id {
            error!(UserEntity::find_by_id(by).one(&self.db).await?.ok_or(RepoError::NotFound)?.role < Role::Moderator => RepoError::Forbidden);
        };
        update!(UserEntity: id => {
            Nickname: nickname,
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    async fn block_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator => RepoError::Forbidden);
        error!(target_user.role != Role::User => RepoError::InvalidRole);
        update!(UserEntity: target_user.id => {
            Role: Role::Blocked,
            Nickname: "_".to_string(),
        })
        .exec(&self.db)
        .await?;
        action!(self; BlockUser@user => json!({"by":by}));
        Ok(())
    }

    async fn unblock_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator => RepoError::Forbidden);
        error!(target_user.role != Role::Moderator => RepoError::InvalidRole);
        update!(UserEntity: target_user.id => {
            Role: Role::User
        })
        .exec(&self.db)
        .await?;
        action!(self; UnblockUser@user => json!({"by":by}));
        Ok(())
    }

    async fn promote_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role != Role::Creator => RepoError::Forbidden);
        error!(target_user.role != Role::User => RepoError::InvalidRole);
        update!(UserEntity: target_user.id => {
            Role: Role::Moderator
        })
        .exec(&self.db)
        .await?;
        action!(self; PromoteUser@user => json!({"by":by}));
        Ok(())
    }

    async fn demote_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role != Role::Creator => RepoError::Forbidden);
        error!(target_user.role != Role::Moderator => RepoError::InvalidRole);
        update!(UserEntity: target_user.id => {
            Role: Role::User
        })
        .exec(&self.db)
        .await?;
        action!(self; DemoteUser@user => json!({"by":by}));
        Ok(())
    }

    async fn get_user(&self, user: i64) -> Result<UserModel, Self::Error> {
        UserEntity::find_by_id(user)
            .one(&self.db)
            .await?
            .ok_or(RepoError::NotFound)
    }

    async fn get_user_by_username(&self, username: String) -> Result<UserModel, Self::Error> {
        UserEntity::find()
            .filter(users::Column::Username.eq(username))
            .one(&self.db)
            .await?
            .ok_or(RepoError::NotFound)
    }

    async fn warn(&self, by: i64, user: i64) -> Result<bool, Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator => RepoError::Forbidden);
        error!(target_user.role > Role::User => RepoError::InvalidRole);
        update!(UserEntity: target_user.id => {
            Warns: target_user.warns+1
        })
        .exec(&self.db)
        .await?;
        action!(self; WarnUser@user => json!({"by":by, "warns":target_user.warns+1}));
        if target_user.warns + 1 >= self.max_warns {
            self.block_user(by, user).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn un_warn(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator => RepoError::Forbidden);
        error!(target_user.role > Role::User => RepoError::InvalidRole);
        if target_user.warns <= 0 {
            Err(RepoError::NotAllowed)?;
        }
        update!(UserEntity: target_user.id => {
            Warns: target_user.warns-1
        })
        .exec(&self.db)
        .await?;
        action!(self; WarnUser@user => json!({"by":by, "warns":target_user.warns-1}));
        Ok(())
    }

    async fn create_command(
        &self,
        name: String,
        action: String,
        creator: i64,
    ) -> Result<(), Self::Error> {
        Command {
            name: Set(name.clone()),
            action: Set(action.clone()),
            creator_id: Set(creator.clone()),
            ..Default::default()
        }
        .insert(&self.db)
        .await?;
        action!(self; CreateCommand@creator => json!({
            "command": name,
            "action": action
        }));
        Ok(())
    }

    async fn update_command(&self, id: String, by: i64, action: String) -> Result<(), Self::Error> {
        let (command, user) = tokio::try_join!(
            CommandEntity::find_by_id(&id).one(&self.db),
            UserEntity::find_by_id(by).one(&self.db),
        )?;
        let command = command.ok_or(RepoError::CommandNotFound)?;
        let user = user.ok_or(RepoError::NotFound)?;
        error!(
            user.role == Role::Blocked ||
            user.id != command.creator_id ||
            user.role < Role::Moderator => RepoError::Forbidden
        );

        update!(CommandEntity where Name: command.name => {
            Action: &action
        })
        .exec(&self.db)
        .await?;
        action!(self; EditCommand@by => json!({
            "command": id,
            "action": action,
        }));
        Ok(())
    }

    async fn delete_command(&self, id: String, by: i64) -> Result<(), Self::Error> {
        let (command, user) = tokio::try_join!(
            CommandEntity::find_by_id(&id).one(&self.db),
            UserEntity::find_by_id(by).one(&self.db),
        )?;
        let command = command.ok_or(RepoError::CommandNotFound)?;
        let user = user.ok_or(RepoError::NotFound)?;
        error!(
            user.id != command.creator_id ||
            user.role < Role::Moderator => RepoError::Forbidden
        );
        CommandEntity::delete_by_id(&id).exec(&self.db).await?;
        action!(self; DeleteCommand@by => json!({
            "command": id,
        }));
        Ok(())
    }

    async fn get_command(&self, id: String) -> Result<CommandModel, Self::Error> {
        CommandEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepoError::CommandNotFound)
    }

    async fn get_user_commands(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<CommandModel>, Self::Error> {
        Ok(CommandEntity::find()
            .filter(commands::Column::CreatorId.eq(user))
            .limit(Some(page))
            .offset(Some(page * page_size))
            .all(&self.db)
            .await?)
    }

    async fn get_commands(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<CommandModel>, Self::Error> {
        Ok(CommandEntity::find()
            .limit(Some(page))
            .offset(Some(page * page_size))
            .all(&self.db)
            .await?)
    }

    async fn use_command(&self, id: String, by: i64) -> Result<(), Self::Error> {
        let user = UserEntity::find_by_id(by)
            .one(&self.db)
            .await?
            .ok_or(RepoError::NotFound)?;
        error!(user.role == Role::Blocked => RepoError::Forbidden);
        let res = update!(CommandEntity where Name: id => {
            TimesUsed: Expr::col(commands::Column::TimesUsed).add(1)
        })
        .exec(&self.db)
        .await?;
        error!(res.rows_affected <= 0 => RepoError::CommandNotFound);
        Ok(())
    }

    async fn new_action(
        &self,
        user_id: i64,
        action_type: Type,
        description: Json,
    ) -> Result<i64, Self::Error> {
        Ok(Action {
            user_id: Set(user_id),
            action_type: Set(action_type),
            description: Set(description),
            ..Default::default()
        }
        .insert(&self.db)
        .await?
        .id)
    }

    async fn get_action(&self, id: i64) -> Result<ActionModel, Self::Error> {
        ActionEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepoError::ActionNotFound)
    }

    async fn get_user_actions(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<ActionModel>, Self::Error> {
        Ok(ActionEntity::find()
            .filter(actions::Column::UserId.eq(user))
            .limit(Some(page))
            .offset(Some(page * page_size))
            .all(&self.db)
            .await?)
    }

    async fn get_actions(&self,page: u64,
                         page_size: u64,) -> Result<Vec<ActionModel>, Self::Error> {
        Ok(ActionEntity::find()
            .limit(Some(page))
            .offset(Some(page * page_size))
            .all(&self.db)
            .await?)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RepoError {
    #[default]
    UnknownError,
    InternalDbError(DbErr),
    NotModified,
    NotFound,
    CommandNotFound,
    ActionNotFound,
    Forbidden,
    InvalidRole,
    NotAllowed,
    AlreadyExists,
}

impl From<DbErr> for RepoError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::RecordNotFound(_) => RepoError::NotFound,
            DbErr::RecordNotInserted | DbErr::RecordNotUpdated => RepoError::NotModified,
            DbErr::Exec(RuntimeErr::SqlxError(sqlx::error::Error::Database(err)))
                if err.is_unique_violation() =>
            {
                RepoError::AlreadyExists
            }
            err => RepoError::InternalDbError(err),
        }
    }
}
