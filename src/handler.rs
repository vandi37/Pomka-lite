mod get;

use crate::repository::db::RepoError;
use crate::repository::RepositoryTrait;

pub struct Handler<R> where  R: RepositoryTrait<Error=RepoError> {
    repo: R,
}

impl<R> Handler<R> where R: RepositoryTrait<Error=RepoError> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

