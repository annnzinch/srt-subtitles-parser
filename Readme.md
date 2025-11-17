# Srt Subtitles Parser

## Links

Crate: https://crates.io/crates/srt_subtitles_parser
Docs: https://docs.rs/srt_subtitles_parser

## Brief Description

Srt Subtitles Parser is a Rust-based parser that processes `.srt` (SubRip Subtitle) files. The parser reads `.srt` files, validates their structure, and extracts subtitle entries consisting of index number, a start timestamp, an end timestamp, and one or more lines of subtitle text. The parser converts the file into a structured data format, which can be used for:

- Converting subtitles to other formats such as WebVTT, JSON, CSV.
- Performing time-based analysis (total duration, reading speed, gaps detection)
- Validating subtitle file consistency (sequential numbering, non-overlapping timestamps)
- Filtering, searching, or manipulating subtitle text
- Synchronizing subtitles by shifting timecodes

## Parsing Process

### What is Being Parsed

The parser processes SRT subtitle files with the following structure:

```
1
00:00:00,000 --> 00:00:02,500
Welcome to the Example Subtitle File!

2
00:00:03,000 --> 00:00:06,000
This is a demonstration of SRT subtitles.

```

Each subtitle entry consists of:

- **Index**: sequential number identifying the subtitle
- **Timecode**: start and end times in format `HH:MM:SS,mmm --> HH:MM:SS,mmm`
  - hours: 00-99
  - minutes: 00-59
  - seconds: 00-59
  - milliseconds: 000-999
- **Text**: one or more lines of text content
- **Separator**: empty line between entries

### Grammar Overview
The parser uses Pest grammar with the following rules:

* **WHITESPACE:**
a whitespace character, which can be a space or a tab

```
WHITESPACE = _{ " " | "\t" }
```

* **NEWLINE:**
handles line breaks

```
NEWLINE = _{ "\r\n" | "\n" }
```

* **index:**
index number (integer)

```
index = @{ ASCII_DIGIT+ }
```

* **hours, minutes, seconds, milliseconds:**
components of timestamp, each with fixed width

```
hours = @{ ASCII_DIGIT{2} }
minutes = @{ ASCII_DIGIT{2} }
seconds = @{ ASCII_DIGIT{2} }
milliseconds = @{ ASCII_DIGIT{3} }
```

* **timestamp:**
time in HH:MM:SS,mmm format

```
timestamp = { hours ~ ":" ~ minutes ~ ":" ~ seconds ~ "," ~ milliseconds }
```

* **timecode:**
start and end timestamps separated by `" --> "`.

```
timecode = { timestamp ~ WHITESPACE* ~ "-->" ~ WHITESPACE* ~ timestamp }
```

* **text_line:**
single line of subtitle text (cannot be empty)

```
text_line = @{ (!NEWLINE ~ ANY)+ }
```

* **text_content:**
subtitle content, which can span multiple lines

```
text_content = { text_line ~ (NEWLINE ~ text_line)* }
```

* **subtitle_block:**
a complete subtitle entry: index, timecode, text, and mandatory blank line

```
subtitle_block = { 
    index ~ NEWLINE ~ 
    timecode ~ NEWLINE ~ 
    text_content ~ NEWLINE ~ 
    NEWLINE
}
```

* **subtitle_file:**
a full subtitle file containing one or more subtitle blocks.

```
subtitle_file = { 
    SOI ~ 
    (subtitle_block)+ ~
    NEWLINE* ~ 
    EOI 
}
```

### Parsing Process

The parsing process includes:

1. **Reading**: input .srt file path
2. **Tokenization**: splitting input into subtitle blocks using Pest grammar rules
3. **Extracting**: parsing each block to extract: index, start and end timestamps, and text content
4. **Validating**: checking format, valid time ranges, presence of required blank lines and block structure completeness
5. **Transforming**: parsing data into a structured Rust types (Subtitle, Timestamp, SubtitleFile)

### Data Structures

The parser produces the following structured data:

```
pub struct SubtitleFile {
    pub subtitles: Vec<Subtitle>,
}

pub struct Subtitle {
    pub index: u32,
    pub start: Timestamp,
    pub end: Timestamp,
    pub text: String,
}

pub struct Timestamp {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}
```

### How Results Are Used

The structured subtitle data can be used for:

- **Serialization**: conversion to JSON using Serde
- **Deserialization**: conversion from JSON using Serde
- **Text Analysis**: extracting text for translation or word count
- **Quality Control**: detecting timing errors, missing indices, or overlapping subtitles
- **Statistics**: calculating total duration, average subtitle length, reading speed
- **Timecode Manipulation**: shifting all timestamps by a fixed offset
- **Time Conversion**: converting timestamps to/from milliseconds for calculations


### Example Input

```
1
00:00:00,000 --> 00:00:02,500
Welcome to the Example Subtitle File!

2
00:00:03,000 --> 00:00:06,000
This is a demonstration of SRT subtitles.

```

### Example Output

```
{
  "subtitles": [
    {
      "index": 1,
      "start": {
        "hours": 0,
        "minutes": 0,
        "seconds": 0,
        "milliseconds": 0
      },
      "end": {
        "hours": 0,
        "minutes": 0,
        "seconds": 2,
        "milliseconds": 500
      },
      "text": "Welcome to the Example Subtitle File!"
    },
    {
      "index": 2,
      "start": {
        "hours": 0,
        "minutes": 0,
        "seconds": 3,
        "milliseconds": 0
      },
      "end": {
        "hours": 0,
        "minutes": 0,
        "seconds": 6,
        "milliseconds": 0
      },
      "text": "This is a demonstration of SRT subtitles."
    }
  ]
}
```