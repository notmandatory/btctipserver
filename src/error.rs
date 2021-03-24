use std::fmt;

#[derive(Debug)]
pub enum Error {
    IniNotFound,
    MissingBDKSection,
    MissingParameter(String),
    InvalidNetwork,

    Sled(bdk::sled::Error),
    Bdk(bdk::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IniNotFound => write!(
                f,
                "Can't find 'config.ini', did you edit and rename 'config_example.ini'?"
            ),
            Error::MissingBDKSection => write!(f, "Missing BDK section in config.ini"),
            Error::MissingParameter(p) => write!(f, "Missing parameter '{}' in config.ini", p),
            Error::InvalidNetwork => write!(
                f,
                "Invalid network value, possible value are 'bitcoin', 'testnet' or 'regtest'"
            ),
            Error::Sled(e) => write!(f, "{}", e),
            Error::Bdk(e) => write!(f, "{}", e),
        }
    }
}

macro_rules! impl_error {
    ( $from:ty, $to:ident ) => {
        impl std::convert::From<$from> for Error {
            fn from(err: $from) -> Self {
                Error::$to(err)
            }
        }
    };
}

impl_error!(bdk::sled::Error, Sled);
impl_error!(bdk::Error, Bdk);
