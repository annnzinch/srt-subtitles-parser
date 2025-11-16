use std::fs;
use srt_subtitles_parser::{parse_srt, SubtitleFile, SrtSubtitleParser, Rule};
use anyhow::Result;
use pest::Parser;

fn main() -> Result<()> {
    println!("Starting program...");

    let path = "./test_file.srt";
    let srt_content = fs::read_to_string(path)?;
    println!("File successfully read! Content:\n{}\n", srt_content);

    let subtitles: SubtitleFile = parse_srt(&srt_content)?;

    println!("Parsed subtitles:\n");

    for subtitle in &subtitles.subtitles {
        println!("Index: {}", subtitle.index);
        println!("Start: {}", subtitle.start);
        println!("End: {}", subtitle.end);
        println!("Text:\n{}\n", subtitle.text);
        println!("----------------------");
    }

    Ok(())
}
