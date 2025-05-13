use crate::models::{commands, prelude::*, users};
use crate::{error, update};
use sea_orm::{
    sea_query::SimpleExpr,
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QuerySelect,
};

pub struct Repository {
    db: DatabaseConnection,
    max_warns: i64,
}

impl Repository {
    fn new(db: DatabaseConnection, max_warns: i64) -> Self {
        Self { db, max_warns }
    }
}

impl super::Repository for Repository {
    type Error = RepoError;

    async fn new_user(&self, role: Role) -> Result<i64, Self::Error> {
        Ok(User {
            role: Set(role),
            ..Default::default()
        }
        .insert(&self.db)
        .await?
        .id)
    }

    async fn block_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator -> RepoError::Forbidden);
        error!(target_user.role != Role::User -> RepoError::InvalidRole);
        update!(UserEntity @target_user.id => {
            Role: Role::Blocked
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    async fn unblock_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator -> RepoError::Forbidden);
        error!(target_user.role != Role::Moderator -> RepoError::InvalidRole);
        update!(UserEntity @target_user.id => {
            Role: Role::User
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    async fn promote_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role != Role::Creator -> RepoError::Forbidden);
        error!(target_user.role != Role::User -> RepoError::InvalidRole);
        update!(UserEntity @target_user.id => {
            Role: Role::Moderator
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    async fn demote_user(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role != Role::Creator -> RepoError::Forbidden);
        error!(target_user.role != Role::Moderator -> RepoError::InvalidRole);
        update!(UserEntity @target_user.id => {
            Role: Role::User
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    async fn get_user(&self, user: i64) -> Result<UserModel, Self::Error> {
        UserEntity::find_by_id(user)
            .one(&self.db)
            .await?
            .ok_or(RepoError::NotFound)
    }

    async fn warn(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator -> RepoError::Forbidden);
        error!(target_user.role > Role::User -> RepoError::InvalidRole);
        update!(UserEntity @target_user.id => {
            Warns += 1
        })
        .exec(&self.db)
        .await?;
        if target_user.warns + 1 >= self.max_warns {
            self.block_user(by, user).await
        } else {
            Ok(())
        }
    }

    async fn un_warn(&self, by: i64, user: i64) -> Result<(), Self::Error> {
        let (by_user, target_user) = tokio::try_join!(
            UserEntity::find_by_id(by).one(&self.db),
            UserEntity::find_by_id(user).one(&self.db),
        )?;
        let by_user = by_user.ok_or(RepoError::NotFound)?;
        let target_user = target_user.ok_or(RepoError::NotFound)?;
        error!(by_user.role < Role::Moderator -> RepoError::Forbidden);
        error!(target_user.role > Role::User -> RepoError::InvalidRole);
        if target_user.warns <= 0 {
            Err(RepoError::NotAllowed)?;
        }
        update!(UserEntity @target_user.id => {
            Warns -= 1
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    async fn create_command(&self, command: CommandModel) -> Result<(), Self::Error> {
        Command {
            name: Set(command.name),
            action: Set(command.action),
            creator_id: Set(command.creator_id),
            ..Default::default()
        }
        .insert(&self.db)?
    }

    async fn update_command(&self, id: String, by: i64, action: String) -> Result<(), Self::Error> {
        let (command, user) = tokio::try_join!(
            CommandEntity::find_by_id(id).one(&self.db),
            UserEntity::find_by_id(by).one(&self.db),
        )?;
        let command = command.ok_or(RepoError::NotFound)?;
        let user = user.ok_or(RepoError::NotFound)?;
        error!(
            user.role == Role::Blocked ||
            user.id == command.creator_id ||
            user.role < Role::Moderator -> RepoError::Forbidden
        );

        update!(CommandEntity @command.name => {
            Action: action
        })
        .exec(&self.db)
        .await?;
        Ok(())
    }

    async fn delete_command(&self, id: String, by: i64) -> Result<(), Self::Error> {
        let (command, user) = tokio::try_join!(
            CommandEntity::find_by_id(id).one(&self.db),
            UserEntity::find_by_id(by).one(&self.db),
        )?;
        let command = command.ok_or(RepoError::NotFound)?;
        let user = user.ok_or(RepoError::NotFound)?;
        error!(
            user.id == command.creator_id ||
            user.role < Role::Moderator -> RepoError::Forbidden
        );
        CommandEntity::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    async fn get_command(&self, id: String) -> Result<CommandModel, Self::Error> {
        CommandEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(RepoError::NotFound)
    }

    async fn get_user_commands(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<CommandModel>, Self::Error> {
        Ok(CommandEntity::find()
            .filter(commands::Column::CreatorId.eq(user))
            .limit(Some(page as u64))
            .offset(Some(page * page_size))
            .all(&self.db)
            .await?)
    }

    async fn use_command(&self, id: String, by: i64) -> Result<(), Self::Error> {
        todo!()
    }

    async fn new_action(&self, action: ActionModel) -> Result<(), Self::Error> {
        todo!()
    }

    async fn get_action(&self, id: i64) -> Result<ActionModel, Self::Error> {
        todo!()
    }

    async fn get_user_actions(
        &self,
        user: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<ActionModel>, Self::Error> {
        todo!()
    }
}

pub enum RepoError {
    InternalDbError(DbErr),
    Quite,
    NotFound,
    Forbidden,
    InvalidRole,
    NotAllowed,
}

impl From<DbErr> for RepoError {
    fn from(value: DbErr) -> Self {
        match value {
            DbErr::RecordNotFound(_) => RepoError::NotFound,
            DbErr::RecordNotInserted | DbErr::RecordNotUpdated => RepoError::Quite,
            err => RepoError::InternalDbError(err),
        }
    }
}
