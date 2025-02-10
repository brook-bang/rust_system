use std::{
    fmt::{self, Display}, path::PathBuf, str::FromStr, time::{Duration, Instant}
};

use crate::error::ImagixError;

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

#[derive(Debug)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{}ns", n),
            (0, n) if n < 1000_000 => write!(out, "{}us", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

impl FromStr for SizeOption {
    type Err = ImagixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Ok(SizeOption::Small),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Single,
    All,
}

impl FromStr for Mode {
    type Err = ImagixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagixError::UserInputError(
                "Wrong value for moda".to_string(),
            )),
        }
    }
}

pub fn processing_resize_request(
    size: SizeOption,
    mode: Mode,
    src_folder: &mut PathBuf,
) -> Result<(),ImagixError> {
    let size = match size {
        SizeOption::Small => 200,
        SizeOption::Medium => 400,
        SizeOption::Large => 800,
    };

    let _ = match mode {
        Mode::All => resize_all
        
    };

}

fn resize_all(size: u32, src_folder: &mut PathBuf) -> Result<(),ImagixError> {
    if let Ok(entries) = get_image_
}


