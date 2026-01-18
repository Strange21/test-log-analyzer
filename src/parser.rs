use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Default, Debug)]
pub struct LogCount {
    pub error: u64,
    pub info: u64,
    pub warn: u64,
    pub malformed: u64,
}

#[derive(PartialEq, Debug)]
pub enum LineErr<'a> {
    MissingField,
    EmptyLevel,
    MalformedChars { pos: usize, ch: char },
    UnknownLevel(&'a str),
}

impl<'a> fmt::Display for LineErr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineErr::MissingField => write!(f, "Missing field(s)"),
            LineErr::EmptyLevel => write!(f, "empty level field"),
            LineErr::MalformedChars { pos, ch } => {
                write!(f, "malformed character '{}' at position {}", ch, pos)
            }
            LineErr::UnknownLevel(l) => write!(f, "unknown level found {}", l),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    Info,
    Warn,
    Error,
}

/// Parse level from a log line like:
/// <timestamp>|<level>|<service>|<message>
/// Returns Ok(Level) for known levels, Err(LineErr) for malformed / unknown.
pub fn parse_level<'a>(line: &'a str) -> Result<Level, LineErr<'a>> {
    let mut parts = line.splitn(4, '|');
    let _ts = parts.next();
    let level = parts.next().ok_or(LineErr::MissingField)?;
    let _svc = parts.next();

    let lvl = level.trim();
    if lvl.is_empty() {
        return Err(LineErr::EmptyLevel);
    }

    // Detect malformed characters in the level field: control or non-ASCII
    for (i, ch) in lvl.char_indices() {
        if ch.is_control() || !ch.is_ascii() {
            return Err(LineErr::MalformedChars { pos: i, ch });
        }
    }

    if lvl.eq("ERROR") {
        Ok(Level::Error)
    } else if lvl.eq("INFO") {
        Ok(Level::Info)
    } else if lvl.eq("WARN") {
        Ok(Level::Warn)
    } else {
        Err(LineErr::UnknownLevel(lvl))
    }
}