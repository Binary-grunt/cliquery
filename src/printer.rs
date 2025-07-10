/// Displays a response to the user with a border or simple format.
pub fn print_response(response: &str) {
    println!("\n==================== Response ====================");
    println!("{}", response.trim());
    println!("==================================================\n");
}

/// Displays a standard error message
pub fn print_error(msg: &str) {
    eprintln!("[Error] {}", msg);
}

/// Displays a standard informational message
pub fn print_info(msg: &str) {
    println!("[Info] {}", msg);
}
