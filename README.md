# idk-shell

[![Rust](https://img.shields.io/badge/rust-1.90.0-brightgreen)](https://www.rust-lang.org/)
[![CI](https://github.com/uictorius/you-sayed-idk-/actions/workflows/ci.yml/badge.svg)](https://github.com/uictorius/you-sayed-idk-/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub Release](https://img.shields.io/github/v/release/uictorius/you-sayed-idk-?label=release)](https://github.com/uictorius/you-sayed-idk-/releases)

A **Rust-based command-line shell** that integrates system command execution (like `ls` and `echo`) with **unique text recognition features** (like variations of `"idk"`).

---

## Features

- **Shell Functionality:** Executes external system commands and basic internal commands (`echo`, `ls`).
- **Custom Recognition:** Maintains legacy recognition for "idk" variations (e.g., `idk`, `idka`, `idkdepressivo`).
- Multi-language support (i18n) for system messages.
- Colored console output with a bold prompt (`idk@shell:~$`).
- Pre-commit hooks for formatting and linting.
- Easy extension for new languages.

---

## Supported Languages

| Language             | Code | File           |
| -------------------- | ---- | -------------- |
| Arabic               | `ar` | `i18n/ar.toml` |
| Bengali              | `bn` | `i18n/bn.toml` |
| English              | `en` | `i18n/en.toml` |
| Spanish              | `es` | `i18n/es.toml` |
| Hindi                | `hi` | `i18n/hi.toml` |
| Japanese             | `ja` | `i18n/ja.toml` |
| Punjabi              | `pa` | `i18n/pa.toml` |
| Portuguese           | `pt` | `i18n/pt.toml` |
| Russian              | `ru` | `i18n/ru.toml` |
| Chinese (Simplified) | `zh` | `i18n/zh.toml` |

> The system automatically detects the OS language. Default fallback is English (`en`).

---

## Project Structure

```

you-sayed-idk-/
├── Cargo.lock
├── Cargo.toml
├── i18n
│   ├── ar.toml
│   ├── bn.toml
│   ├── en.toml
│   ├── es.toml
│   ├── hi.toml
│   ├── ja.toml
│   ├── pa.toml
│   ├── pt.toml
│   ├── ru.toml
│   └── zh.toml
├── LICENSE
├── README.md
└── src
    ├── commands.rs
    ├── main.rs
    ├── print.rs
    ├── prompt.rs
    └── translations.rs

```

- `src/main.rs` → main entry point
- `i18n/` → translation files for shell messages
- `src/` → modularized code (shell commands, checks, printing, prompts, translations)

---

## Build & Run

**Using Cargo (recommended):**

```bash
# Build in release mode
cargo build --release

# Run the project
cargo run
```

**Usage Example:**

```bash
idk@shell:~$ echo Hello, world!
Hello, world!
idk@shell:~$ idk_mode idk
You said 'idk'.
```

---

## Development Workflow

1. **Format code**:

```bash
cargo fmt
```

2. **Lint code**:

```bash
cargo clippy
```

3. **Use Conventional Commits**:

- `feat:` → new feature
- `fix:` → bug fix
- `chore:` → project structure/tooling
- `style:` → formatting changes
- `refactor:` → refactoring without behavior change

---

## Git Hooks (Automatic Formatting & Linting)

To enforce code quality, the project uses versioned **pre-commit hooks** in `.githooks/pre-commit`.
These hooks automatically run `cargo fmt` and `cargo clippy`.

**Setup:**

```bash
git config core.hooksPath .githooks
```

---

## Releasing a New Version

Uses [`cargo-release`](https://github.com/crate-ci/cargo-release) for versioning and publishing.

```bash
# Install cargo-release if needed
cargo install cargo-release

# Dry-run a release
cargo release 2.0.0 --dry-run

# Release the version
cargo release 2.0.0
```

**Notes:**

- Ensure all changes are committed
- Confirm `Cargo.toml` includes `description`, `license`, and `repository`
- Run `cargo fmt` and `cargo clippy` before releasing

---

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit using Conventional Commits
4. Push and open a Pull Request

---

## Credits

Original project by [thz-afk](https://github.com/thz-afk/you-sayed-idk-).
Enhanced with multi-language support and modular architecture.
