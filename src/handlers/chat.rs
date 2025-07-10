use crate::printer::print_response;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;
use std::env;

/// Handles a prompt in a named chat session.
pub fn handle_chat(prompt: &str, session: &str) {
    if prompt.trim().is_empty() {
        eprintln!("[Error] Empty prompt provided.");
        return;
    }

    let chat_dir = get_chat_cache_dir();
    let session_file = chat_dir.join(format!("{}.log", session));

    let mut history = String::new();

    if session_file.exists() {
        if let Ok(contents) = read_to_string(&session_file) {
            history = contents;
        }
    }

    let simulated_response = format!("assistant: (Simulated) response to '{}'.", prompt);

    // Display response
    print_response(&simulated_response);

    // Update session file
    let updated_history = format!("{}\nuser: {}\n{}", history.trim(), prompt.trim(), simulated_response);
    if let Err(e) = write(&session_file, updated_history) {
        eprintln!("[Error] Failed to save session: {}", e);
    }
}

/// Returns fallback chat directory at $HOME/.cache/cliquery/chat_cache
fn get_chat_cache_dir() -> PathBuf {
    let mut path = env::var("HOME").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from("/tmp"));
    path.push(".cache/cliquery/chat_cache");
    let _ = create_dir_all(&path);
    path
}
