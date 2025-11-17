use anyhow::{Context, Result};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Parser for SRT subtitle files using Pest grammar
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SrtSubtitleParser;

/// Errors
#[derive(Error, Debug)]
pub enum SrtError {
    #[error("Failed to parse SRT file")]
    ParseError(#[from] anyhow::Error),

    #[error("Missing expected component in subtitle block: {0}")]
    MissingComponent(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Failed to parse index: {0}")]
    InvalidIndex(String),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Represents a full subtitle file containing multiple subtitle entries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubtitleFile {
    pub subtitles: Vec<Subtitle>,
}

/// Represents a single subtitle entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subtitle {
    pub index: u32,
    pub start: Timestamp,
    pub end: Timestamp,
    pub text: String,
}

/// Represents a single subtitle timestamp with hours, minutes, seconds, and milliseconds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timestamp {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}

impl Timestamp {
    /// Converts timestamp to milliseconds
    pub fn to_ms(&self) -> u64 {
        (self.hours as u64 * 3600000)
            + (self.minutes as u64 * 60000)
            + (self.seconds as u64 * 1000)
            + (self.milliseconds as u64)
    }

    /// Creates timestamp from milliseconds
    pub fn from_ms(ms: u64) -> Self {
        let hours = (ms / 3600000) as u32;
        let minutes = ((ms % 3600000) / 60000) as u32;
        let seconds = ((ms % 60000) / 1000) as u32;
        let milliseconds = (ms % 1000) as u32;

        Timestamp {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl SubtitleFile {
    /// Converts SubtitleFile to json
    pub fn to_json(&self) -> Result<String, SrtError> {
        serde_json::to_string_pretty(self).map_err(SrtError::JsonError)
    }

    /// Creates SubtitleFile from json
    pub fn from_json(json: &str) -> Result<Self, SrtError> {
        serde_json::from_str(json).map_err(SrtError::JsonError)
    }

    /// Converts SubtitleFile back to srt
    pub fn to_srt(&self) -> String {
        self.subtitles
            .iter()
            .map(|s| format!("{}\n{} --> {}\n{}\n\n", s.index, s.start, s.end, s.text))
            .collect::<String>()
    }

    /// Shifts subtitles
    pub fn shift_time(&mut self, offset_ms: i64) {
        for subtitle in &mut self.subtitles {
            let start_ms = subtitle.start.to_ms() as i64 + offset_ms;
            let end_ms = subtitle.end.to_ms() as i64 + offset_ms;

            subtitle.start = Timestamp::from_ms(start_ms.max(0) as u64);
            subtitle.end = Timestamp::from_ms(end_ms.max(0) as u64);
        }
    }
}

/// Parses an SRT file content into a SubtitleFile struct
pub fn parse_srt(input: &str) -> Result<SubtitleFile, SrtError> {
    let pairs =
        SrtSubtitleParser::parse(Rule::subtitle_file, input).context("Failed to parse SRT file")?;

    let mut subtitles = Vec::new();

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::subtitle_block => {
                    subtitles.push(parse_subtitle_block(inner_pair)?);
                }
                Rule::EOI => {}
                _ => {}
            }
        }
    }

    Ok(SubtitleFile { subtitles })
}

/// Parses a single subtitle block into a Subtitle struct
pub fn parse_subtitle_block(pair: pest::iterators::Pair<Rule>) -> Result<Subtitle, SrtError> {
    let mut inner = pair.into_inner();

    //index
    let index_pair = inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("index".into()))?;
    let index: u32 = index_pair
        .as_str()
        .parse()
        .map_err(|_| SrtError::InvalidIndex(index_pair.as_str().into()))?;

    //timecode
    let timecode_pair = inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("timecode".into()))?;
    let mut timecode_inner = timecode_pair.into_inner();

    let start_timestamp = timecode_inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("start timestamp".into()))?;
    let start = parse_timestamp(start_timestamp)?;

    let end_timestamp = timecode_inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("end timestamp".into()))?;
    let end = parse_timestamp(end_timestamp)?;

    //text_content
    let text_pair = inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("text content".into()))?;
    let text = parse_text(text_pair);

    Ok(Subtitle {
        index,
        start,
        end,
        text,
    })
}

/// Parses a timestamp pair into a Timestamp struct
pub fn parse_timestamp(pair: pest::iterators::Pair<Rule>) -> Result<Timestamp, SrtError> {
    let mut inner = pair.into_inner();

    let hours = inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("hours".into()))?
        .as_str()
        .parse()
        .map_err(|_| SrtError::InvalidTimestamp("hours".into()))?;
    let minutes = inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("minutes".into()))?
        .as_str()
        .parse()
        .map_err(|_| SrtError::InvalidTimestamp("minutes".into()))?;
    let seconds = inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("seconds".into()))?
        .as_str()
        .parse()
        .map_err(|_| SrtError::InvalidTimestamp("seconds".into()))?;
    let milliseconds = inner
        .next()
        .ok_or_else(|| SrtError::MissingComponent("milliseconds".into()))?
        .as_str()
        .parse()
        .map_err(|_| SrtError::InvalidTimestamp("milliseconds".into()))?;

    Ok(Timestamp {
        hours,
        minutes,
        seconds,
        milliseconds,
    })
}

/// Converts a text_content pair into a single string, joining lines with \n
pub fn parse_text(pair: pest::iterators::Pair<Rule>) -> String {
    pair.into_inner()
        .map(|p| p.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}
