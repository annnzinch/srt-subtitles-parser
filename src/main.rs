use srt_subtitles_parser::*;
use std::fs;

fn main() -> anyhow::Result<()> {
    //srt file parsing
    let srt_content = fs::read_to_string("test_file.srt")?;
    let mut subtitles = parse_srt(&srt_content)?;
    
    println!("Parsed {} subtitles", subtitles.subtitles.len());

    //converting to json
    let json = subtitles.to_json()?;
    fs::write("output.json", json)?;
    println!("Saved to JSON");

    //time shift +2 seconds
    subtitles.shift_time(2000);
    
    //converting back to srt
    let new_srt = subtitles.to_srt();
    fs::write("shifted.srt", new_srt)?;
    println!("Saved shifted subtitles");

    Ok(())
}