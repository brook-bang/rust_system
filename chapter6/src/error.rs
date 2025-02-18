use core::fmt;
use std::{fmt::Display, io, num::TryFromIntError};

#[derive(Debug)]
pub struct StatsError {
    pub message: String,
}

impl Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) ->Result<(),fmt::Error> {
        write!(f,"{}",self.message)
    }
}

impl From<&str> for StatsError {
    fn from(s: &str) -> Self {
        StatsError {
            message: s.to_string(),
        }
    }
}

impl From<io::Error> for StatsError {
    fn from(e: io::Error) -> Self {
        StatsError {
            message: e.to_string()
        }
    }
}

impl From<TryFromIntError> for StatsError {
    fn from(_e: TryFromIntError) -> Self {
        StatsError {
            message: "Number conversion error".to_string(),
        }
    }
}