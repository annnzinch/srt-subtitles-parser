use srt_subtitles_parser::*;
use anyhow::Result;

const SINGLE_BLOCK: &str = "26
00:02:13,383 --> 00:02:14,883
Carol moved her stuff out today.";

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
my brother, Ross?";

#[test]
fn test_parse_index() -> Result<()> {
    let subtitle = parse_subtitle_block(SINGLE_BLOCK)?;
    assert_eq!(subtitle.index, 26);
    Ok(())
}

#[test]
fn test_parse_timecode() -> Result<()> {
    let subtitle = parse_subtitle_block(SINGLE_BLOCK)?;
    assert_eq!(subtitle.start_time, "00:02:13,383");
    assert_eq!(subtitle.end_time, "00:02:14,883");
    Ok(())
}

#[test]
fn test_parse_text_single_line() -> Result<()> {
    let subtitle = parse_subtitle_block(SINGLE_BLOCK)?;
    assert_eq!(subtitle.text, vec!["Carol moved her stuff out today."]);
    Ok(())
}

#[test]
fn test_parse_text_multi_line() -> Result<()> {
    let subtitle = parse_subtitle_block(MULTI_LINE_BLOCK)?;
    assert_eq!(subtitle.text, vec!["- Let me get you some coffee.", "- Thanks."]);
    Ok(())
}

#[test]
fn test_parse_file_multiple_blocks() -> Result<()> {
    let subtitles = parse_file(FULL_FILE)?;
    assert_eq!(subtitles.len(), 2);
    assert_eq!(subtitles[0].index, 55);
    assert_eq!(subtitles[1].index, 56);
    Ok(())
}
