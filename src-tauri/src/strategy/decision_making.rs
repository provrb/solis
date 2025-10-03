use std::fs;

use crate::core::TelemetryPacket;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

const MODEL_NAME: &str = "llama3";
const SYS_PROMPT_FILE_PATH: &str = "./system_prompt.txt";

pub async fn init() -> Ollama {
    let ollama = Ollama::default();
    ollama
        .pull_model(MODEL_NAME.to_string(), false)
        .await
        .expect("failed to pull ollama model");

    // system prompt
    if let Ok(system_prompt_text) = fs::read_to_string(SYS_PROMPT_FILE_PATH) {
        ollama.generate(
            GenerationRequest::new(MODEL_NAME.to_string(), "System prompt:")
                .system(system_prompt_text),
        );
    }

    ollama
}

pub fn answer_question(model: &Ollama, message: String, telemetry_context: Vec<TelemetryPacket>) {}
