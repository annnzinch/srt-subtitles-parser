use srt_subtitles_parser::*;
use pest::Parser;

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
fn ok(rule: Rule, input: &str) {
    SrtSubtitleParser::parse(rule, input)
        .unwrap_or_else(|_| panic!("Should parse: {:?}", input));
}

#[track_caller]
fn err(rule: Rule, input: &str) {
    assert!(
        SrtSubtitleParser::parse(rule, input).is_err(),
        "Should NOT parse: {:?}",
        input
    );
}

//index
#[test]
fn test_index() {
    ok(Rule::index, "1");
    ok(Rule::index, "42");
    err(Rule::index, "");
    err(Rule::index, "abc");
}

//timestamps
#[test]
fn test_hours() {
    ok(Rule::hours, "00");
    ok(Rule::hours, "12");
    err(Rule::hours, "1");
}

#[test]
fn test_minutes() {
    ok(Rule::minutes, "00");
    ok(Rule::minutes, "59");
    err(Rule::minutes, "6");
}

#[test]
fn test_seconds() {
    ok(Rule::seconds, "00");
    ok(Rule::seconds, "59");
    err(Rule::seconds, "7");
}

#[test]
fn test_milliseconds() {
    ok(Rule::milliseconds, "000");
    ok(Rule::milliseconds, "500");
    err(Rule::milliseconds, "1");
    err(Rule::milliseconds, "12");
}

#[test]
fn test_timestamp() {
    ok(Rule::timestamp, "00:00:02,500");
    ok(Rule::timestamp, "12:34:56,789");

    err(Rule::timestamp, "0:00:00,000");
    err(Rule::timestamp, "00:00:00.000");
}

#[test]
fn test_timecode() {
    ok(Rule::timecode, "00:00:00,000 --> 00:00:02,500"); 
    ok(Rule::timecode, "01:02:03,004-->04:05:06,007"); 

    err(Rule::timecode, "00:00:00,000 > 00:00:02,500");
}

//text
#[test]
fn test_text_line() {
    ok(Rule::text_line, "Hello world");
    ok(Rule::text_line, "123");
    ok(Rule::text_line, "Symbols! @#*&^");
    err(Rule::text_line, "");
}

#[test]
fn test_text_content() {
    ok(Rule::text_content, "Hello");
    ok(Rule::text_content, "Hello\nworld"); 
    ok(Rule::text_content, "Line 1\nLine 2\nLine 3");
    ok(Rule::text_content, "Line 1\nLine 2\n");
    err(Rule::text_content, "\n");
    err(Rule::text_content, "");
}

//subtitle block
#[test]
fn test_subtitle_block_single() {
    ok(Rule::subtitle_block, SINGLE_BLOCK);
    err(Rule::subtitle_block, "26
00:02:13,383 --> 00:02:14,883
Carol moved her stuff out today.");
}

#[test]
fn test_subtitle_block_multi_line() {
    ok(Rule::subtitle_block, MULTI_LINE_BLOCK);
}

//file
#[test]
fn test_file() {
    ok(Rule::subtitle_file, FULL_FILE);
    ok(Rule::subtitle_file, SINGLE_BLOCK); 
    ok(Rule::subtitle_file, &format!("{}\n", SINGLE_BLOCK)); 
    ok(Rule::subtitle_file, &format!("{}{}", SINGLE_BLOCK, MULTI_LINE_BLOCK));
}

#[test]
fn test_file_missing_blank_line() {
    err(Rule::subtitle_file, "1
00:00:00,000 --> 00:00:02,500
Text without blank line
2
00:00:03,000 --> 00:00:05,000
Another subtitle");
}

#[test]
fn test_empty_text_line() {
    err(Rule::text_line, "");
}