# transcript-segmenter

Group a flat ASR transcript into **logical paragraphs** — silence gaps, speaker
changes and sentence boundaries — with correct spacing for Latin and CJK text.

Pure Rust, no I/O, no async runtime. One dependency: `serde`.

```text
ASR backend ──▶ [ TranscriptSegment ] ──▶ split_into_paragraphs() ──▶ [ Paragraph ]
 whisper.cpp        start / end / text        silence  ▸ speaker           text / start
 faster-whisper     speaker / confidence      ▸ sentence ▸ cap           end / speaker
 cloud API
```

Open-source reference implementation extracted from my commercial work on a
B2B workspace platform (proprietary business logic removed).

## Why

Every streaming ASR backend — `whisper.cpp`, `faster-whisper`, `whisper.cpp`
via the cloud — emits the same shape: a flat list of timed segments. That is
fine for karaoke-style highlighting, but it reads badly in a document. A human
writes paragraphs; a transcript arrives as a stream of fragments.

This crate converts the flat list into paragraphs using three signals, in
priority order:

| Signal | Condition | Rationale |
|---|---|---|
| **Silence** | gap between segments `> silence_break_seconds` | the speaker paused → new thought |
| **Speaker** | labels present and different, `break_on_speaker` | someone else started talking |
| **Sentence** | text ends with `.!?。！？…` **and** gap `>= sentence_break_gap_seconds` | natural paragraph end |

Plus a hard ceiling: `max_paragraph_chars` stops a talker who never pauses from
producing a wall of text. The cap only fires at a sentence boundary or a pause,
so paragraphs still end somewhere sensible.

## Install

```toml
[dependencies]
transcript-segmenter = "0.1"
```

## Usage

```rust
use transcript_segmenter::{split_into_paragraphs, SegmentationOptions, TranscriptionSegment};

let segments = vec![
    TranscriptionSegment { start: 0.0, end: 1.2, text: "So the first thing we tried".into(),
                           speaker: None, confidence: 0.94 },
    TranscriptionSegment { start: 1.2, end: 2.4, text: "was rewriting the ingestion layer.".into(),
                           speaker: None, confidence: 0.91 },
    TranscriptionSegment { start: 4.8, end: 6.0, text: "That turned out to matter a lot.".into(),
                           speaker: None, confidence: 0.97 },
];

for p in split_into_paragraphs(&segments, &SegmentationOptions::default()) {
    println!("[{:.1}s - {:.1}s] {}", p.start, p.end, p.text);
}
```

Output:

```text
[0.0s - 2.4s] So the first thing we tried was rewriting the ingestion layer.
[4.8s - 6.0s] That turned out to matter a lot.
```

Run the bundled demo:

```bash
cargo run --example demo
```

## Options

```rust
SegmentationOptions {
    silence_break_seconds: 1.5,      // silence gap that starts a new paragraph
    sentence_break_gap_seconds: 0.4, // pause after a full stop that starts a new paragraph
    break_on_speaker: true,          // split when the speaker label changes
    max_paragraph_chars: 600,        // hard ceiling per paragraph
}
```

All four are `Copy`, so one `SegmentationOptions` can be reused across calls.

## CJK-aware joining

Segments are joined with a space, **except** when either side is a CJK
character — CJK text is written without inter-word spaces, so inserting one
corrupts it:

```text
"今天" + "天气很好。"  →  "今天天气很好。"
"hello" + "world"     →  "hello world"
```

The detection covers CJK Unified Ideographs, Extension A, CJK punctuation,
fullwidth forms and general punctuation.

## Design notes

- **Pure.** No clock, no filesystem, no network. The function is deterministic
  for a given input, which makes it trivially testable and safe to run
  concurrently.
- **No `unwrap` on user input.** The only `expect` is on a `Vec` the function
  itself just checked for emptiness.
- **Character count, not byte count.** Caps are measured in `chars()`, so a
  paragraph of CJK text is capped by visible width rather than by UTF-8 byte
  length.
- **Timing is preserved.** Each `Paragraph` carries the `start` of its first
  segment and the `end` of its last, so downstream code can still seek, align
  to audio, or attach captions.

## Testing

```bash
cargo test
```

10 unit tests cover: empty input, silence splitting, sentence-end splitting,
the character cap, speaker-change splitting, the disabled-speaker path, CJK
joining, Latin joining, timestamp spanning, and blank-segment skipping.

## License

MIT
