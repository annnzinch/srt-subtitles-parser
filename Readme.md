# Srt Subtitles Parser

## Links

Crate: https://crates.io/crates/srt_subtitles_parser
Docs: 

## Brief Description

Srt Subtitles Parser is a Rust-based parser that processes `.srt` (SubRip Subtitle) files. The parser reads `.srt` files validates their structure, and extracts subtitle entries consisting of index number, a start timestamp, an end timestamp and one or more lines of subtitle text. The parser converts the file into a structured data format, which can be used for:

- Converting subtitles to other formats such as (WebVTT, JSON, CSV)
- Performing time-based analysis (total duration, reading speed, gaps detection)
- Validating subtitle file consistency (sequential numbering, non-overlapping timestamps)
- Filtering, searching, or manipulating subtitle text
- Synchronizing subtitles by shifting timecodes

## Parsing Process

### What is Being Parsed

The parser processes SRT subtitle files with the following structure:

```
1
00:00:01,000 --> 00:00:04,000
First line

2
00:00:05,500 --> 00:00:08,000
Second line
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

* **WHITESPACE:**
a whitespace character, which can be a space or a tab.

```
WHITESPACE = _{ " " | "\t" }
```

* **NEWLINE:**
handles line breaks.

```
NEWLINE = _{ "\r\n" | "\n" }
```

* **index:**
index number (integer).

```
index = { ASCII_DIGIT+ }
```

* **timestamp:**
time in `HH:MM:SS,mmm` format.

```
timestamp = { ASCII_DIGIT ~ ASCII_DIGIT ~ ":" ~ ASCII_DIGIT ~ ASCII_DIGIT ~ ":" ~ ASCII_DIGIT ~ ASCII_DIGIT ~ "," ~ ASCII_DIGIT ~ ASCII_DIGIT ~ ASCII_DIGIT }
```

* **timecode:**
start and end timestamps separated by `" --> "`.

```
timecode = { timestamp ~ WHITESPACE* ~ "-->" ~ WHITESPACE* ~ timestamp }
```

* **text_line** / **text:**
subtitle content, which can span multiple lines.

```
text_line = { (!NEWLINE ~ ANY)* }
text = { text_line ~ (NEWLINE ~ text_line)* }
```

* **subtitle_block:**
a complete subtitle entry: index, timecode, and text.

```
subtitle_block = { index ~ NEWLINE ~ timecode ~ NEWLINE ~ text ~ (NEWLINE+ | EOI) }
```

* **file:**
a full subtitle file containing one or more subtitle blocks.

```
file = { subtitle_block+ }
```

### Parsing Process

The parsing process includes:

1. **Reading**: input file path or raw .srt text
2. **Tokenization**: splitting input into subtitle blocks using Pest grammar rules
3. **Extracting**: parsing each block to extract: index, start and end timestamps and text content
4. **Validating**: checking timestamps (format, valid time ranges, non-overlapping) and block order
5. **Transforming**: parsing data into a structured format (Subtitle structs)

### How Results Are Used

The structured subtitle data enables:

- **Format Conversion**: export to JSON, WebVTT, or CSV formats
- **Text Analysis**: extract text for translation or word count
- **Quality Control**: detect timing errors, missing indices, or overlapping subtitles
- **Statistics**: calculate total duration, average subtitle length, reading speed
- **Timecode Manipulation**: shift all timestamps by a fixed offset


### Example Input

### Example Output