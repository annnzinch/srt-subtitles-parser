use anyhow::Result;
use srt_subtitles_parser::{SubtitleFile, parse_srt};
use std::fs;
use std::io::{self, Write};

fn main() -> Result<()> {
    println!("SRT Parser");
    println!("Type 'help' for available commands");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let mut parts = input.split_whitespace();
        let command = parts.next();

        match command {
            Some("parse-to-json") => {
                if let (Some(file), Some(output)) = (parts.next(), parts.next()) {
                    let content = fs::read_to_string(file)?;
                    let subtitles = parse_srt(&content)?;
                    fs::write(output, subtitles.to_json()?)?;
                    println!("JSON saved to '{}'", output);
                } else {
                    println!("Usage: parse-to-json <file.srt> <output.json>");
                }
            }
            Some("parse-to-srt") => {
                if let (Some(file), Some(output)) = (parts.next(), parts.next()) {
                    let content = fs::read_to_string(file)?;
                    let subtitles = SubtitleFile::from_json(&content)?;
                    fs::write(output, subtitles.to_srt())?;
                    println!("SRT saved to '{}'", output);
                } else {
                    println!("Usage: parse-to-srt <file.json> <output.srt>");
                }
            }
            Some("shift-time") => {
                if let (Some(file), Some(offset_str), Some(output)) =
                    (parts.next(), parts.next(), parts.next())
                {
                    let offset: i64 = offset_str.parse()?;
                    let content = fs::read_to_string(file)?;
                    let mut subtitles = parse_srt(&content)?;
                    subtitles.shift_time(offset);
                    fs::write(output, subtitles.to_srt())?;
                    println!("Shifted SRT saved to '{}'", output);
                } else {
                    println!("Usage: shift-time <file.srt> <offset_ms> <output.srt>");
                }
            }
            Some("credits") => {
                println!("Author: Anna Zinchenko\nVersion: 0.1.0");
            }
            Some("help") => {
                println!("Available commands:");
                println!("parse-to-json <file.srt> <output.json>           - Convert SRT to JSON");
                println!("parse-to-srt <file.json> <output.srt>           - Convert JSON to SRT");
                println!(
                    "shift-time <file.srt> <offset_ms> <output.srt>  - Shift timestamps by offset"
                );
                println!("credits                                         - Show author info");
                println!("exit                                            - Quit program");
            }
            Some("exit") => break,
            Some(other) => println!("Unknown command: {}", other),
            None => continue,
        }
    }

    Ok(())
}
