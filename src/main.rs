use srt_subtitles_parser::*;
use std::io::{self, Write};
use std::fs;

fn main() {
    loop {
        println!("\nSRT Subtitles Parser\n");
        println!("Select an action:");
        println!("1. Convert SRT to JSON");
        println!("2. Convert JSON to SRT");
        println!("3. Shift timestamps");
        println!("4. Show credits");
        println!("5. Exit");
        print!("Enter choice [1-5]: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => srt_to_json(),
            "2" => json_to_srt(),
            "3" => shift_timestamps(),
            "4" => show_credits(),
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn srt_to_json() {
    let path = read_file_path("Enter path to SRT file: ");
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    match parse_srt(&content) {
        Ok(subs) => {
            match subs.to_json() {
                Ok(json) => {
                    let out_file = "output.json";
                    fs::write(out_file, json).ok();
                    println!("Parsed {} subtitles.", subs.subtitles.len());
                    println!("JSON saved to '{}'", out_file);
                }
                Err(e) => eprintln!("Serialization error: {}", e),
            }
        }
        Err(e) => eprintln!("Parse error: {}", e),
    }
}

fn json_to_srt() {
    let path = read_file_path("Enter path to JSON file: ");
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    match SubtitleFile::from_json(&content) {
        Ok(subs) => {
            let out_file = "output.srt";
            fs::write(out_file, subs.to_srt()).ok();
            println!("Parsed {} subtitles.", subs.subtitles.len());
            println!("SRT saved to '{}'", out_file);
        }
        Err(e) => eprintln!("Deserialization error: {}", e),
    }
}

fn shift_timestamps() {
    let path = read_file_path("Enter path to SRT or JSON file: ");
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut subs = if path.ends_with(".json") {
        match SubtitleFile::from_json(&content) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Deserialization error: {}", e);
                return;
            }
        }
    } else {
        match parse_srt(&content) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Parse error: {}", e);
                return;
            }
        }
    };

    println!("Enter offset in milliseconds (positive or negative): ");
    let mut offset_str = String::new();
    io::stdin().read_line(&mut offset_str).unwrap();
    let offset_ms: i64 = match offset_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number.");
            return;
        }
    };

    subs.shift_time(offset_ms);

    println!("Choose output format:");
    println!("1. SRT");
    println!("2. JSON");
    print!("Enter choice [1-2]: ");
    io::stdout().flush().unwrap();
    let mut fmt_choice = String::new();
    io::stdin().read_line(&mut fmt_choice).unwrap();
    let fmt_choice = fmt_choice.trim();

    match fmt_choice {
        "1" => {
            let out_file = "shifted_output.srt";
            fs::write(out_file, subs.to_srt()).ok();
            println!("Shifted SRT saved to '{}'", out_file);
        }
        "2" => {
            match subs.to_json() {
                Ok(json) => {
                    let out_file = "shifted_output.json";
                    fs::write(out_file, json).ok();
                    println!("Shifted JSON saved to '{}'", out_file);
                }
                Err(e) => eprintln!("Serialization error: {}", e),
            }
        }
        _ => println!("Invalid choice, skipping output."),
    }
}

fn show_credits() {
    println!("SRT Subtitles Parser v{}", env!("CARGO_PKG_VERSION"));
    println!("Author: {}", env!("CARGO_PKG_AUTHORS"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));

}

fn read_file_path(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut path = String::new();
    io::stdin().read_line(&mut path).unwrap();
    path.trim().to_string()
}
