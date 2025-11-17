use anyhow::{Context, Result, bail};
use pest::Parser;
use srt_subtitles_parser::*;

const SINGLE_BLOCK: &str = "26
00:02:13,383 --> 00:02:14,883
Carol moved her stuff out today.

";

const MULTI_LINE_BLOCK: &str = "28
00:02:16,302 --> 00:02:18,095
- Let me get you some coffee.
- Thanks.

";

const FULL_FILE: &str = "55
00:03:52,440 --> 00:03:54,900
This is everybody.
This is Chandler and Phoebe...

56
00:03:55,109 --> 00:03:57,361
...and Joey. And remember
my brother, Ross?

";

#[track_caller]
fn ok(rule: Rule, input: &str) -> Result<()> {
    SrtSubtitleParser::parse(rule, input).with_context(|| format!("Should parse: {:?}", input))?;
    Ok(())
}

#[track_caller]
fn err(rule: Rule, input: &str) -> Result<()> {
    if SrtSubtitleParser::parse(rule, input).is_ok() {
        bail!("Should NOT parse: {:?}", input);
    }
    Ok(())
}

//index
#[test]
fn test_index() -> Result<()> {
    ok(Rule::index, "1")?;
    ok(Rule::index, "42")?;
    err(Rule::index, "")?;
    err(Rule::index, "abc")?;
    Ok(())
}

//timestamps
#[test]
fn test_hours() -> Result<()> {
    ok(Rule::hours, "00")?;
    ok(Rule::hours, "12")?;
    err(Rule::hours, "1")?;
    Ok(())
}

#[test]
fn test_minutes() -> Result<()> {
    ok(Rule::minutes, "00")?;
    ok(Rule::minutes, "59")?;
    err(Rule::minutes, "6")?;
    Ok(())
}

#[test]
fn test_seconds() -> Result<()> {
    ok(Rule::seconds, "00")?;
    ok(Rule::seconds, "59")?;
    err(Rule::seconds, "7")?;
    Ok(())
}

#[test]
fn test_milliseconds() -> Result<()> {
    ok(Rule::milliseconds, "000")?;
    ok(Rule::milliseconds, "500")?;
    err(Rule::milliseconds, "1")?;
    err(Rule::milliseconds, "12")?;
    Ok(())
}

#[test]
fn test_timestamp() -> Result<()> {
    ok(Rule::timestamp, "00:00:02,500")?;
    ok(Rule::timestamp, "12:34:56,789")?;

    err(Rule::timestamp, "0:00:00,000")?;
    err(Rule::timestamp, "00:00:00.000")?;
    Ok(())
}

#[test]
fn test_timecode() -> Result<()> {
    ok(Rule::timecode, "00:00:00,000 --> 00:00:02,500")?;
    ok(Rule::timecode, "01:02:03,004-->04:05:06,007")?;

    err(Rule::timecode, "00:00:00,000 > 00:00:02,500")?;
    Ok(())
}

//text
#[test]
fn test_text_line() -> Result<()> {
    ok(Rule::text_line, "Hello world")?;
    ok(Rule::text_line, "123")?;
    ok(Rule::text_line, "Symbols! @#*&^")?;
    err(Rule::text_line, "")?;
    Ok(())
}

#[test]
fn test_text_content() -> Result<()> {
    ok(Rule::text_content, "Hello")?;
    ok(Rule::text_content, "Hello\nworld")?;
    ok(Rule::text_content, "Line 1\nLine 2\nLine 3")?;
    ok(Rule::text_content, "Line 1\nLine 2\n")?;
    err(Rule::text_content, "\n")?;
    err(Rule::text_content, "")?;
    Ok(())
}

//subtitle block
#[test]
fn test_subtitle_block_single() -> Result<()> {
    ok(Rule::subtitle_block, SINGLE_BLOCK)?;
    err(
        Rule::subtitle_block,
        "26
        00:02:13,383 --> 00:02:14,883
        Carol moved her stuff out today.",
    )?;
    Ok(())
}

#[test]
fn test_subtitle_block_multi_line() -> Result<()> {
    ok(Rule::subtitle_block, MULTI_LINE_BLOCK)?;
    Ok(())
}

//file
#[test]
fn test_file() -> Result<()> {
    ok(Rule::subtitle_file, FULL_FILE)?;
    ok(Rule::subtitle_file, SINGLE_BLOCK)?;
    ok(Rule::subtitle_file, &format!("{}\n", SINGLE_BLOCK))?;
    ok(
        Rule::subtitle_file,
        &format!("{}{}", SINGLE_BLOCK, MULTI_LINE_BLOCK),
    )?;
    Ok(())
}

#[test]
fn test_file_missing_blank_line() -> Result<()> {
    err(
        Rule::subtitle_file,
        "1
        00:00:00,000 --> 00:00:02,500
        Text without blank line
        2
        00:00:03,000 --> 00:00:05,000
        Another subtitle",
    )?;
    Ok(())
}

#[test]
fn test_empty_text_line() -> Result<()> {
    err(Rule::text_line, "")?;
    Ok(())
}
