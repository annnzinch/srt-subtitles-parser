use pest::Parser;
use pest_derive::Parser;
use anyhow::{Result, Context};

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct SrtSubtitleParser;

#[derive(Debug, Clone)]
pub struct Subtitle {
    pub index: u32,
    pub start_time: String,
    pub end_time: String,
    pub text: Vec<String>,
}

//subtitle block parsing
pub fn parse_subtitle_block(input: &str) -> Result<Subtitle> {
    let mut pairs = SrtSubtitleParser::parse(Rule::subtitle_block, input)
        .context("Failed to parse the subtitle block")?;

    let block = pairs.next().context("No subtitle block found")?;

    let mut index: u32 = 0;
    let mut start_time = String::new();
    let mut end_time = String::new();
    let mut text_lines: Vec<String> = Vec::new();

    for pair in block.into_inner() {
        match pair.as_rule() {
            Rule::index => {
                index = pair.as_str().parse::<u32>()
                    .context("Failed to parse the index")?;
            }
            Rule::timecode => {
                let mut timestamps = pair.into_inner();
                let start = timestamps.next().context("No start timestamp")?;
                let end = timestamps.next().context("No end timestamp")?;
                start_time = start.as_str().to_string();
                end_time = end.as_str().to_string();
            }
            Rule::text => {
                for line in pair.into_inner().filter(|p| p.as_rule() == Rule::text_line) {
                    let s = line.as_str();
                    if !s.trim().is_empty() {
                        text_lines.push(s.to_string());
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Subtitle {
        index,
        start_time,
        end_time,
        text: text_lines,
    })
}


//file parsing
pub fn parse_file(input: &str) -> Result<Vec<Subtitle>> {
    let mut subtitles = Vec::new();

    for block_text in input.split("\n\n") {
        if !block_text.trim().is_empty() {
            subtitles.push(parse_subtitle_block(block_text)?);
        }
    }

    Ok(subtitles)
}