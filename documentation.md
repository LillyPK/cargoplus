# Cargo Plus Documentation

## Table of Contents

* [Overview](#overview)
* [Creating a New Project](#creating-a-new-project)
* [Understanding `vaulter.toml`](#understanding-vaultersoml)
* [Vaulting Constants](#vaulting-constants)
* [Executable Icons](#executable-icons)
* [Building Your Project](#building-your-project)
* [Working With Existing Cargo Commands](#working-with-existing-cargo-commands)

---

## Overview

Cargo Plus is a developer tool designed to enhance Rust workflows. It wraps and extends Cargo with additional functionality like:

* Automatic inclusion of icons
* Vaulting constants to avoid tampering
* Easy setup via `vaulter.toml`

---

## Creating a New Project

Run the following:

```bash
cargoplus new your_project_name
```

You will be prompted to choose:

* Whether to use checksum-protected constants
* Whether to add an icon file (default: `icon.ico`)

This will generate a project structure with:

* `Cargo.toml`
* `src/main.rs`
* `vaulter.toml`
* `icon.ico`

---

## Understanding `vaulter.toml`

The `vaulter.toml` file controls which constants in which files will be protected. Example:

```toml
[main.rs]
api_key = true
password = true

[config.rs]
secret = true
```

This tells Cargo Plus to hash the contents of these constants and verify their integrity at runtime.

---

## Vaulting Constants

When you run:

```bash
cargoplus build
```

Cargo Plus:

1. Parses `vaulter.toml`
2. Locates matching constant values in your `.rs` files
3. Hashes their values and generates runtime checksum verification logic

Any tampering (e.g., using a hex editor) will cause the program to fail checksum verification.

---

## Executable Icons

When prompted during project creation, Cargo Plus can embed a provided icon (like `icon.ico`) into the executable. This works across supported platforms, especially Windows and Linux desktop environments.

You can replace `icon.ico` with any `.ico` or `.png` file and re-run `cargoplus build`.

---

## Building Your Project

Use the following commands:

```bash
cargoplus build        # Debug build with vaulting + icon
cargoplus build --release   # Release build with vaulting + icon
```

For normal development:

```bash
cargo run              # No vaulting or icon injection
```

---

## Working With Existing Cargo Commands

`cargoplus` does not interfere with `cargo run`, `cargo test`, or other standard commands.

* Use `cargo` for local development.
* Use `cargoplus` when you want security, icon inclusion, or production-ready builds.

---

Happy building with Cargo Plus!
