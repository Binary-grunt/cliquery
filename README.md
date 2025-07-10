# 🛡️ cliquery

> A privacy-first, Rust-native terminal assistant inspired by [ShellGPT](https://github.com/TheR1D/shell_gpt), built for Linux.
> Designed for long-term maintainability, zero dependencies, and offline-first AI workflows.

---

## 🚀 About

**cliquery** is a command-line interface assistant, originally inspired by the excellent [ShellGPT by TheR1D](https://github.com/TheR1D/shell_gpt). While ShellGPT focuses on convenient OpenAI API usage in terminal, cliquery takes a different stance:

🛑 No remote AI calls
🔐 Security first
🦀 Written entirely in Rust
📦 Zero external dependencies (std-only)
🧱 Full control over your assistant, your data, and your system

---

## 🎯 Philosophy

cliquery is designed with one goal: build a secure, inspectable, long-living CLI assistant that never talks to the internet unless explicitly allowed.
It is not a wrapper around APIs — it is a framework for safe, local interaction with language models and command pipelines.

Key principles:

* ✅ **Security by default**: no remote HTTP calls, no shell execution, no unsafe reflection
* 🧰 **Unix composability**: each function is CLI-first, modular, and testable
* 🧼 **Minimalism**: no serde, no clap, no dependencies unless absolutely needed
* 💡 **Hackable**: built to be forked, read, and audited line-by-line
* 🔒 **Privacy-aware AI isolation** (future): planned Podman containerization, local LLM engine support
* 📆 **Long-term durability**: intended for >10 years maintenance

---

## 🖥️ Platform Support

* 🐧 Linux: ✅ supported & tested (Wayland, systemd, Fedora/NixOS/Debian)
* 🪟 Windows: ❌ not supported yet — contributions welcome in the future
* 🍏 macOS: ❌ not supported yet — contributions welcome in the future

---

## 🔐 Future Plans

cliquery is only the foundation. Coming next:

* ✅ `--version`, `--help`, `--config`, `--list-chats`, `--list-roles`
* 🔒 AI Isolation: full LLM sandboxing via Podman
* 🔌 Plug-in system for LLM backends (e.g., llama.cpp, Ollama) — still entirely local
* 🧰 Automated setup CLI: `cliquery init --offline`
* 📄 Manual pages (man 1 cliquery)
* 🧪 Fuzz-testing + command validation pipelines
* 📁 Structured output parsing (mode --code, --shell)
* 🧠 Long-term memory and multi-session cache rotation
* ✍️ Prompt chaining, role scripting, and programmable agents

All while maintaining a strict "no internet unless requested" policy.

---

## 🏗️ Project Structure (modular, Unix-like) still in progress.

```
cliquery/
├── src/
│   ├── bin/                 # 🔹 Entry point binaries (default: cliquery.rs)
│   │   └── cliquery.rs      # Main command-line executable
│   ├── lib.rs               # 🔹 Public API exposure (for testing/integration)
│   ├── cli/                 # 🔹 CLI mode dispatch (parses --chat, --repl, etc.)
│   │   ├── repl.rs          # Interactive REPL session mode
│   │   ├── chat.rs          # Persistent session with cache
│   │   └── default.rs       # One-shot prompt mode (stdin/arg)
│   ├── engine/              # 🔹 Core logic ("AI brain" and processing)
│   │   ├── prompt.rs        # Prompt construction & formatting
│   │   ├── sandbox.rs       # Podman/Fork/Namespace isolation layer (planned)
│   │   └── llm.rs           # Simulated or embedded LLM backend (future)
│   ├── store/               # 🔹 Filesystem state: config, cache, roles
│   │   ├── config.rs        # Loads ~/.config/cliquery/.cfg
│   │   ├── cache.rs         # Stores chat sessions in ~/.cache/cliquery/
│   │   └── roles.rs         # Handles role prompts from ~/.config/cliquery/roles/
│   └── ui/                  # 🔹 User-facing output logic
│       └── printer.rs       # Console output: formatting, color, warnings
```

---

## 🔍 Security Model

* 🧱 No runtime shell execution
* ❌ No external crate injection
* 🛡️ No network activity unless added explicitly by user
* ✅ Filesystem access confined to:

  * `~/.config/cliquery/`
  * `~/.cache/cliquery/`
* ✍️ Future Podman integrations will isolate AI execution in hardened containers (AppArmor/seccomp capable)

---

## ⚠️ Disclaimer

This project is under active development. Do not rely on it for production yet.
That said, the goal is to be your long-term assistant — predictable, secure, and quiet.

---

## 📜 License

MIT. Built for freedom, learning, and paranoia.

---
