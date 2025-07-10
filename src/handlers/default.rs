use crate::printer::print_response;

/// Handles a basic prompt passed as an argument, without REPL or chat session.
pub fn handle_default(prompt: &str) {
    // 🔒 Simulation: here we could call a local engine (not included)
    // Example of mocked logic (to be replaced with a real AI call or internal plugin)
    if prompt.trim().is_empty() {
        eprintln!("[Error] Empty prompt provided.");
        return;
    }

    // 🔧 Dummy logic – to be replaced:
    let fake_response = format!("(Simulation) AI response to: '{}'", prompt);
    print_response(&fake_response);
}
