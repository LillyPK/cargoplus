# Cargo Plus

### A developer-friendly Rust project manager and secure constant vaulting tool.

---

## 📖 Description

**Cargo Plus** is a powerful Rust command-line tool that extends `cargo` with additional features like secure vaulting of constants, executable icon embedding, and streamlined project scaffolding. It's designed for developers who want to protect sensitive values (like API keys) in compiled binaries without editing their source files.

---

## ✨ Features

* `cargoplus new <project-name>` — Initializes a new Rust project with optional vaulting and an icon.
* `vaulter.toml` — Declare constant variable protection with an easy-to-edit configuration file.
* Seamless icon embedding (cross-platform support).
* Works alongside regular `cargo` commands like `cargo run`.
* Adds powerful `cargoplus build` and `cargoplus build --release` commands for production builds.
* First-time setup prompt to symlink `cargoplus` to `/usr/local/bin` on Linux.

---

## 🚀 Quick Start Guide

After installing `cargoplus`, simply open a terminal and use:

```bash
cargoplus new my_project
```

You will be prompted:

* Do you want to use checksum-protected constants? (Y/n)
* (Optional) Do you want to include an executable icon?

Once completed, your project directory will look like:

```
my_project/
├── Cargo.toml
├── icon.ico
├── src/
│   └── main.rs
└── vaulter.toml
```

Then you can start building your project:

```bash
cargoplus build
cargoplus build --release
```

To run your project in development, use:

```bash
cargo run
```
