mod config;
mod handlers;
mod printer;

use handlers::{chat::handle_chat, default::handle_default, repl::handle_repl};
use std::env;
use std::process;

fn print_usage() {
    eprintln!("Usage: cliquery [--chat NAME] [--repl NAME] [PROMPT...]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Case: no argument provided
    if args.len() == 1 {
        print_usage();
        process::exit(1);
    }

    let mut i = 1;
    let mut chat_mode: Option<String> = None;
    let mut repl_mode: Option<String> = None;
    let mut prompt_parts: Vec<String> = vec![];

    while i < args.len() {
        match args[i].as_str() {
            "--chat" => {
                i += 1;
                if i < args.len() {
                    chat_mode = Some(args[i].clone());
                } else {
                    eprintln!("Error: --chat requires a session name");
                    process::exit(1);
                }
            }
            "--repl" => {
                i += 1;
                if i < args.len() {
                    repl_mode = Some(args[i].clone());
                } else {
                    eprintln!("Error: --repl requires a session name");
                    process::exit(1);
                }
            }
            val => {
                prompt_parts.push(val.to_string());
            }
        }
        i += 1;
    }

    let prompt = prompt_parts.join(" ");

    if let Some(session) = chat_mode {
        handle_chat(&prompt, &session);
    } else if let Some(session) = repl_mode {
        handle_repl(&prompt, &session);
    } else {
        handle_default(&prompt);
    }
}
