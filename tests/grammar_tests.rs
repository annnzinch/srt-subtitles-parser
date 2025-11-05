use anyhow::anyhow;
use srt_subtitles_parser::*;
use pest::Parser;

#[test]
fn index_single_digit_test() -> anyhow::Result<()> {
    let pair = SrtSubtitleParser::parse(Rule::index, "82")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(pair.as_str(), "82");
    Ok(())
}