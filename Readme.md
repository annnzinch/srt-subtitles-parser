# Srt Subtitles Parser

## Brief Description

SrtSubtitleParser is a Rust-based parser that processes .srt (SubRip Subtitle) files. The parser reads .srt files with entries consisting of an index number, a start timestamp, an end timestamp, and one or more lines of subtitle text. The parser converts the file into a structured data format, which can be used for:

- Converting subtitles to other formats such as .vtt or JSON
- Performing time-based analysis, such as calculating total subtitle duration or average line length
- Validating subtitle file consistency, including sequential numbering and non-overlapping timestamps
- Filtering, searching, or manipulating subtitle text programmatically

## Parsing Process

The parsing process includes:

1. **Reading** .srt text input
2. **Extracting** subtitle blocks (index, time range, and text)
3. **Validating** timestamps and block order
4. **Transforming** parsed data into a structured format (Subtitle structs)