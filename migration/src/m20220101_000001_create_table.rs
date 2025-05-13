use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Role).string().not_null())
                    .col(
                        ColumnDef::new(Users::Warns)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Commands::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Commands::Name)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Commands::Action).string().not_null())
                    .col(ColumnDef::new(Commands::CreatorId).big_integer().not_null())
                    .col(
                        ColumnDef::new(Commands::TimesUsed)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(Commands::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_commands_creator_id")
                            .from(Commands::Table, Commands::CreatorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Actions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Actions::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Actions::UserId).big_integer().not_null())
                    .col(ColumnDef::new(Actions::ActionType).string().not_null())
                    .col(ColumnDef::new(Actions::Description).json().not_null())
                    .col(
                        ColumnDef::new(Actions::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_actions_user_id")
                            .from(Actions::Table, Actions::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Actions::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Commands::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Role,
    Warns,
    CreatedAt,
}
#[derive(Iden)]
enum Commands {
    Table,
    Name,
    Action,
    CreatorId,
    TimesUsed,
    CreatedAt,
}

#[derive(Iden)]
enum Actions {
    Table,
    Id,
    UserId,
    ActionType,
    Description,
    CreatedAt,
}
