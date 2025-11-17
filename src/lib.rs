use pest::Parser;
use pest_derive::Parser;
use anyhow::{Result, Context};
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SrtSubtitleParser;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubtitleFile {
    pub subtitles: Vec<Subtitle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subtitle {
    pub index: u32,
    pub start: Timestamp,
    pub end: Timestamp,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timestamp {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}

impl Timestamp {
    //convert timestamp to millisconds
    pub fn to_ms(&self) -> u64 {
        (self.hours as u64 * 3600000) +
        (self.minutes as u64 * 60000) +
        (self.seconds as u64 * 1000) +
        (self.milliseconds as u64)
    }

    //create timestamp from milliseconds
    pub fn from_ms(ms: u64) -> Self {
        let hours = (ms / 3600000) as u32;
        let minutes = ((ms % 3600000) / 60000) as u32;
        let seconds = ((ms % 60000) / 1000) as u32;
        let milliseconds = (ms % 1000) as u32;
        
        Timestamp { hours, minutes, seconds, milliseconds }
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
    //convert to json
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .context("Failed to serialize to JSON")
    }

    //create from json
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json)
            .context("Failed to deserialize from JSON")
    }

    //convert back to srt
    pub fn to_srt(&self) -> String {
        self.subtitles.iter()
            .map(|s| format!(
                "{}\n{} --> {}\n{}\n\n",
                s.index, s.start, s.end, s.text
            ))
            .collect::<String>()
    }

    //shift subtitles
    pub fn shift_time(&mut self, offset_ms: i64) {
        for subtitle in &mut self.subtitles {
            let start_ms = subtitle.start.to_ms() as i64 + offset_ms;
            let end_ms = subtitle.end.to_ms() as i64 + offset_ms;
            
            subtitle.start = Timestamp::from_ms(start_ms.max(0) as u64);
            subtitle.end = Timestamp::from_ms(end_ms.max(0) as u64);
        }
    }
}

pub fn parse_srt(input: &str) -> Result<SubtitleFile> {
    let pairs = SrtSubtitleParser::parse(Rule::subtitle_file, input)
        .context("Failed to parse SRT file")?;

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

fn parse_subtitle_block(pair: pest::iterators::Pair<Rule>) -> Result<Subtitle> {
    let mut inner = pair.into_inner();

    //index
    let index_pair = inner.next()
        .context("Missing index in subtitle block")?;
    let index: u32 = index_pair.as_str().parse()
        .context("Failed to parse index")?;

    //timecode
    let timecode_pair = inner.next()
        .context("Missing timecode in subtitle block")?;
    let mut timecode_inner = timecode_pair.into_inner();
    
    let start_timestamp = timecode_inner.next()
        .context("Missing start timestamp")?;
    let start = parse_timestamp(start_timestamp)?;
    
    let end_timestamp = timecode_inner.next()
        .context("Missing end timestamp")?;
    let end = parse_timestamp(end_timestamp)?;

    //text_content
    let text_pair = inner.next()
        .context("Missing text content in subtitle block")?;
    let text = parse_text(text_pair);

    Ok(Subtitle {
        index,
        start,
        end,
        text,
    })
}

fn parse_timestamp(pair: pest::iterators::Pair<Rule>) -> Result<Timestamp> {
    let mut inner = pair.into_inner();

    Ok(Timestamp {
        hours: inner.next()
            .context("Missing hours")?
            .as_str().parse()
            .context("Failed to parse hours")?,
        minutes: inner.next()
            .context("Missing minutes")?
            .as_str().parse()
            .context("Failed to parse minutes")?,
        seconds: inner.next()
            .context("Missing seconds")?
            .as_str().parse()
            .context("Failed to parse seconds")?,
        milliseconds: inner.next()
            .context("Missing milliseconds")?
            .as_str().parse()
            .context("Failed to parse milliseconds")?,
    })
}

fn parse_text(pair: pest::iterators::Pair<Rule>) -> String {
    pair.into_inner()
        .map(|p| p.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}