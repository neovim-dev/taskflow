# Contributing to TaskFlow

Thank you for your interest in contributing to TaskFlow! We welcome contributions from everyone, whether you're a seasoned Rustacean or new to open source. This guide outlines the basic process for proposing changes, reporting issues, and submitting pull requests.

## Getting Started

1. **Fork the Repository**
   - Click the "Fork" button on the top right of the [repo page](https://github.com/neovim-dev/taskflow).
   - Clone your fork to your local machine.
   ```
   git clone https://github.com/<your-username>/taskflow.git
   cd taskflow
   ```

2. **Install Dependencies**
   - Ensure you have the latest [Rust toolchain](https://www.rust-lang.org/tools/install).
   - For CLI development, you may need to install additional crates (e.g., `clap`).

3. **Create a Branch**
   - Make a new branch for your feature or fix:
   ```
   git checkout -b my-feature
   ```

## How to Contribute

### 1. Reporting Bugs & Requesting Features

- Please check the [issue tracker](https://github.com/neovim-dev/taskflow/issues) to see if your issue has already been reported.
- Open a new issue for bugs, feature requests, or questions. Use the provided templates for clarity.

### 2. Working on Issues

- Look for issues labeled `good first issue`, `beginner`, or `help wanted` if you're new.
- Ask in the issue comments if you need clarification or want to claim an issue.

### 3. Making Changes

- Follow Rust’s best practices and maintain code formatting with `cargo fmt`.
- Write clear, descriptive commit messages.
- Add or update tests as needed.
- Document new functionality and any important implementation details.

### 4. Pull Requests

- Push your branch to your fork and open a pull request (PR) against `main`.
- Reference related issues in your PR description.
- Ensure that CI checks pass before requesting review.
- Be responsive to feedback and update your PR as needed.

## Coding Guidelines

- Use idiomatic Rust.
- Ensure all public APIs are documented using doc comments (`///`).
- Run `cargo test` to verify that all tests pass before submitting.
- Keep PRs focused; avoid including unrelated changes.

## Code of Conduct

Please be respectful and considerate in all interactions. See our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for more details.

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Ratatui Styling Docs](https://docs.rs/ratatui/latest/ratatui/style/)
- [Chrono Crate Docs](https://docs.rs/chrono/latest/chrono/)
- [Clap Crate Docs](https://docs.rs/clap/)

---

Thank you for helping make TaskFlow better!
