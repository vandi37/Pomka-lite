use teloxide::RequestError;
use crate::repository::db::RepoError;

#[derive(Debug)]
pub enum Error {
    Repo(RepoError),
    Request(RequestError),
}

impl From<RequestError> for Error {
    fn from(err: RequestError) -> Self {
        Error::Request(err)
    }
}

impl From<RepoError> for Error {
    fn from(err: RepoError) -> Self {
        Error::Repo(err)
    }
}