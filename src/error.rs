use crate::common::*;

#[derive(Debug)]
pub enum Error {
  Reqwest{error: reqwest::Error},
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Error {
    Error::Reqwest{error}
  }
}
