// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Error>;

/// Error generated when inspecting the VM.
#[derive(Debug)]
pub enum Error {
    ParseError(String),
    IoError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            IoError(s) => write!(f, "I/O Error, error={}", s),
            ParseError(s) => write!(f, "Failed to parse process information, error={}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseError(e.to_string())
    }
}

#[macro_export]
macro_rules! percentage {
    ($value: expr, $base: expr) => {
        if $value > $base {
            100 as u32
        } else {
            ((($value as f64) / ($base as f64)) * 100.0) as u32
        }
    };
}

pub fn draw_line(dot: char, title: &str, width: usize) -> Option<String> {
    if title.len() > width {
        None
    } else {
        let num_of_dot = width - title.len();
        Some(format!(
            "{}{}{}",
            &vec![dot; num_of_dot / 2].iter().collect::<String>(),
            title,
            &vec![dot; num_of_dot - (num_of_dot / 2)]
                .iter()
                .collect::<String>()
        ))
    }
}

pub trait Inspector {
    fn inspect(&self);
}
