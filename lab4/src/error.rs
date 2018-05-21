use csv::Error as CsvError;
use log::SetLoggerError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;
use std::io::Error as IoError;
use std::num::ParseIntError;

/// Common result type used throughout the program.
/// There aren't really many possible errors though.
pub type Result<T> = StdResult<T, Error>;

/// Common error type used throughout the program, to be used as a holder for
/// errors from various other libraries.
#[derive(Debug)]
pub enum Error {
  /// A custom process error from string
  Paging(String),
  /// A error from parsing an int
  ParseInt(ParseIntError),
  /// A `std::io` module error.
  Io(IoError),
  /// A `log` crate error by `set_logger`.
  SetLogger(SetLoggerError),
  /// A `csv` crate error
  Csv(CsvError),
}

impl<'a> From<&'a str> for Error {
  fn from(err: &'a str) -> Error {
    Error::Paging(err.to_string())
  }
}

impl From<ParseIntError> for Error {
  fn from(err: ParseIntError) -> Error {
    Error::ParseInt(err)
  }
}

impl From<IoError> for Error {
  fn from(err: IoError) -> Error {
    Error::Io(err)
  }
}

impl From<SetLoggerError> for Error {
  fn from(err: SetLoggerError) -> Error {
    Error::SetLogger(err)
  }
}

impl From<CsvError> for Error {
  fn from(err: CsvError) -> Error {
    Error::Csv(err)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    match *self {
      Error::Paging(ref inner) => inner.fmt(f),
      Error::ParseInt(ref inner) => inner.fmt(f),
      Error::Io(ref inner) => inner.fmt(f),
      Error::SetLogger(ref inner) => inner.fmt(f),
      Error::Csv(ref inner) => inner.fmt(f),
    }
  }
}

impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::Paging(ref inner) => inner,
      Error::ParseInt(ref inner) => inner.description(),
      Error::Io(ref inner) => inner.description(),
      Error::SetLogger(ref inner) => inner.description(),
      Error::Csv(ref inner) => inner.description(),
    }
  }
}