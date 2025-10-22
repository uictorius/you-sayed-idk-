# you-sayed-idk

[![Rust](https://img.shields.io/badge/rust-1.90.0-brightgreen)](https://www.rust-lang.org/)
[![CI](https://github.com/uictorius/you-sayed-idk-/actions/workflows/ci.yml/badge.svg)](https://github.com/uictorius/you-sayed-idk-/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/uictorius/you-sayed-idk-?label=release)](https://github.com/uictorius/you-sayed-idk-/releases)

A program to read a line of text and verify if the input is a variation of "idk".

It can identify variations like:

- `idk`, `Idk`, `iDK`, etc.
- `idka`, `ikd!`, etc.
- `idkdepressivo`

---

## Credits

This project is based on the original work by [thz-afk](https://github.com/thz-afk/you-sayed-idk-).

Thank you for creating this amazing tool!

---

## Recommended VSCode Setup

For the best experience, it is recommended to use **VSCode** with these extensions:

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) → language support, formatting, Clippy integration
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) → TOML syntax highlighting
- [Prettier - Code formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode) → auto-format Markdown, JSON, YAML, and other files

Project settings are included in `.vscode/settings.json`:

- Auto-format Rust, TOML, Markdown, JSON, and YAML on save
- Run Clippy on Rust files when saving
- YAML files recognized for `.yml` and `.yaml`

---

## Requirements

- Rust toolchain via [rustup](https://rustup.rs/)
- Recommended tools:
  - `cargo fmt` → format code
  - `cargo clippy` → lint and suggestions

---

## Project Structure

```

you-sayed-idk/
├── Cargo.toml
├── LICENSE
├── README.md
└── src/
└── main.rs

```

- `src/main.rs` → main entry point
- `Cargo.toml` → dependency and project metadata

---

## Build & Run

With Cargo (recommended):

```bash
# Build in release mode
cargo build --release

# Run the project
cargo run
```

Directly with rustc (quick test):

```bash
rustc src/main.rs -o main
./main
```

---

## Development Workflow

1. **Format code on save** (VSCode or manually):

```bash
cargo fmt
```

2. **Lint with Clippy**:

```bash
cargo clippy
```

3. **Follow Conventional Commits**:

- `feat:` → new feature
- `fix:` → bug fix
- `chore:` → project structure, tooling, etc.
- `style:` → formatting changes
- `refactor:` → refactoring without changing behavior

---

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit with Conventional Commits
4. Push and open a Pull Request

---

## License

[MIT](LICENSE)
