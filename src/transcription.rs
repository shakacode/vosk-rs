use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptionWord {
    pub conf: f32,
    pub end: f32,
    pub start: f32,
    pub word: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptionResult {
    pub text: String,
    #[serde(default = "Vec::new")]
    pub result: Vec<TranscriptionWord>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranscriptionPartialResult {
    pub partial: String,
}