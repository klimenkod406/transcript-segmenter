use transcript_segmenter::{split_into_paragraphs, SegmentationOptions, TranscriptionSegment};

fn seg(start: f32, end: f32, text: &str) -> TranscriptionSegment {
    TranscriptionSegment {
        start,
        end,
        text: text.to_string(),
        speaker: None,
        confidence: 1.0,
    }
}

fn main() {
    // A flat list of timed segments, as produced by any ASR backend.
    let segments = vec![
        seg(0.0, 1.2, "So the first thing we tried"),
        seg(1.2, 2.4, "was rewriting the ingestion layer."),
        seg(4.8, 6.0, "That turned out to matter a lot."),
        seg(6.0, 7.1, "The p99 dropped by half."),
    ];

    let options = SegmentationOptions::default();
    let paragraphs = split_into_paragraphs(&segments, &options);

    println!(
        "{} segment(s) -> {} paragraph(s)\n",
        segments.len(),
        paragraphs.len()
    );
    for (i, p) in paragraphs.iter().enumerate() {
        println!("[{i}] {:.1}s - {:.1}s", p.start, p.end);
        println!("    {}\n", p.text);
    }
}
