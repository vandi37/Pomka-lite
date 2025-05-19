use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Default)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub role: Role,
    pub username: Option<String>,
    pub nickname: String,
    pub warns: i64,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, EnumIter, DeriveActiveEnum, Serialize, Deserialize, Default,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::None)",
    rename_all = "snake_case"
)]
pub enum Role {
    Blocked,
    #[default]
    User,
    Moderator,
    Creator,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::actions::Entity")]
    Actions,
    #[sea_orm(has_many = "super::commands::Entity")]
    Commands,
}

impl Related<super::commands::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Commands.def()
    }
}

impl Related<super::actions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Actions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
