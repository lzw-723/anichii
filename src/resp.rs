use rocket::response::Responder;
use rocket::Request;
use serde::de::StdError;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 400)]
    TestQBittorrentError(()),
    #[response(status = 400)]
    InitDatabaseError(()),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::TestQBittorrentError(()) => write!(f, "Test QBittorrentError"),
            Error::InitDatabaseError(()) => write!(f, "Init DatabaseError"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        "TestQBittorrentError"
    }
}
