use crate::models::users::Role;

pub struct RoleSelector {
    creator: Option<i64>,
}

impl RoleSelector {
    pub fn new(creator: Option<i64>) -> Self {
        Self { creator }
    }
    pub fn select(&self, id: i64) -> Role {
        match self.creator {
            Some(creator) if creator == id => Role::Creator,
            _ => Role::User,
        }
    }
}
