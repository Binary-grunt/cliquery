use crate::printer::{print_error, print_response};
use std::io::{self, Write};

/// Starts a REPL session with the given name (temporary or named)
pub fn handle_repl(_init_prompt: &str, session: &str) {
    println!("Entering REPL mode for session '{}'.", session);
    println!("Press Ctrl+C to exit.\n");

    let mut history: Vec<String> = Vec::new();
    loop {
        print!(">>> ");
        if io::stdout().flush().is_err() {
            print_error("Error flushing stdout.");
            break;
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }

                history.push(trimmed.to_string());

                // 💬 Simulated local AI response
                let response = format!("(Simulated) response to: '{}'.", trimmed);
                print_response(&response);
            }
            Err(e) => {
                print_error(&format!("Error reading input: {}", e));
                break;
            }
        }
    }
}
