# TaskFlow - Terminal Todo Manager 🚀

[![Hacktoberfest 2025](https://img.shields.io/badge/Hacktoberfest-2025-blueviolet)](https://hacktoberfest.com/)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A blazingly fast, terminal-based todo list manager built with Rust and Ratatui. Manage your tasks efficiently without leaving your terminal!

## ✨ Features

- 📝 Create, edit, and delete tasks
- ✅ Mark tasks as complete/incomplete
- 🎨 Beautiful TUI (Terminal User Interface) with Ratatui
- 💾 Persistent storage (JSON-based)
- ⌨️ Vim-like keybindings
- 🏷️ Task priorities (High, Medium, Low)
- 📅 Due date tracking
- 🔍 Search and filter tasks
- 📊 Task statistics dashboard

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/neovim-dev/taskflow.git
cd taskflow

# Build the project
cargo build --release

# Run the application
cargo run
```

### Usage

```bash
# Start TaskFlow
taskflow

# Or use the release binary
./target/release/taskflow
```

## ⌨️ Keybindings

| Key | Action |
|-----|--------|
| `n` | New task |
| `e` | Edit selected task |
| `d` | Delete selected task |
| `Space` | Toggle task completion |
| `j/↓` | Move down |
| `k/↑` | Move up |
| `/` | Search tasks |
| `p` | Change priority |
| `q` | Quit application |
| `?` | Show help |

## 📁 Project Structure

```
taskflow/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app.rs            # Application state management
│   ├── ui.rs             # UI rendering logic
│   ├── task.rs           # Task data structures
│   ├── storage.rs        # Data persistence
│   └── input.rs          # Input handling
├── tests/
│   └── integration_tests.rs
├── Cargo.toml
├── README.md
├── CONTRIBUTING.md
├── LICENSE
└── .github/
    └── ISSUE_TEMPLATE/
```

## 🛠️ Technology Stack

- **Language**: Rust
- **TUI Framework**: [Ratatui](https://github.com/ratatui-org/ratatui)
- **Storage**: JSON (serde)
- **Date/Time**: chrono
- **CLI**: clap (for future CLI arguments)

## 🤝 Contributing

We love contributions! This project is part of **Hacktoberfest 2025**. Please read our [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Good First Issues

Check out issues labeled with `good-first-issue` to get started! We have tasks ranging from beginner to advanced levels.

## 📋 Roadmap

- [ ] Basic CRUD operations
- [ ] Task persistence
- [ ] Priority and due dates
- [ ] Search functionality
- [ ] Categories/Tags
- [ ] Task statistics
- [ ] Export to different formats
- [ ] Recurring tasks
- [ ] Multi-list support
- [ ] Cloud sync (optional)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Ratatui](https://github.com/ratatui-org/ratatui)
- Inspired by the terminal tools community
- Thanks to all Hacktoberfest contributors!

## 📞 Support

- 🐛 [Report a Bug](https://github.com/neovim-dev/taskflow/issues/new?template=bug_report.md)
- 💡 [Request a Feature](https://github.com/neovim-dev/taskflow/issues/new?template=feature_request.md)
- 💬 [Join Discussions](https://github.com/neovim-dev/taskflow/discussions)

---

⭐ Star this repository if you find it helpful!

**Happy Hacking! 🎃**
