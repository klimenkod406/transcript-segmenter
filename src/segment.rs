//! A single transcribed segment: the atomic unit produced by any ASR backend.
//!
//! Kept in its own module so downstream consumers can implement their own
//! backends (whisper.cpp, faster-whisper, cloud APIs) without depending on
//! this crate's internals.

use serde::{Deserialize, Serialize};

/// One timed piece of transcribed speech.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptionSegment {
    /// Start offset in seconds, relative to the beginning of the audio.
    pub start: f32,
    /// End offset in seconds.
    pub end: f32,
    /// The recognised text for this segment.
    pub text: String,
    /// Speaker label, when diarization is available.
    pub speaker: Option<String>,
    /// Backend confidence in the range `0.0..=1.0`.
    pub confidence: f32,
}
