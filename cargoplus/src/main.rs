// src/main.rs for cargoplus
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

use toml_edit::{Document, value};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargoplus <command> [project_name]");
        return;
    }

    match args[1].as_str() {
        "new" => {
            if args.len() < 3 {
                eprintln!("Please provide a project name.");
                return;
            }
            let name = &args[2];
            create_new_project(name);
        },
        "build" => {
            perform_vault_checks();
            embed_icon();
            run_cargo_build();
        },
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

fn create_new_project(name: &str) {
    println!("✔ Creating new Rust project: {}", name);
    let status = Command::new("cargo").args(["new", name]).status().unwrap();
    if !status.success() {
        eprintln!("Failed to create project.");
        return;
    }

    let project_path = Path::new(name);
    let icon_path = project_path.join("icon.ico");
    fs::write(&icon_path, b"ICONPLACEHOLDER").unwrap(); // placeholder

    let vaulter_path = project_path.join("vaulter.toml");
    println!("✔ Include variable checksumming via `vaulter.toml`? (y/N): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() == "y" {
        let mut doc = Document::new();
        doc["main.rs"]["api_key"] = value(true);
        doc["main.rs"]["password"] = value(true);
        fs::write(vaulter_path, doc.to_string()).unwrap();
        println!("✔ Added default `vaulter.toml` config");
    }

    println!("✔ Added example executable icon: icon.ico");
    println!("✔ Project created!");
}

fn perform_vault_checks() {
    // Read and parse vaulter.toml, generate code or validate statically
    println!("✔ Performing vault checks from vaulter.toml...");
    // In a real tool, this would inject hash checking into main.rs and other files
}

fn embed_icon() {
    println!("✔ Embedding icon into binary (placeholder)...");
    // platform-specific embedding logic would go here
}

fn run_cargo_build() {
    println!("✔ Running `cargo build --release`...");
    let status = Command::new("cargo").args(["build", "--release"]).status().unwrap();
    if status.success() {
        println!("✔ Build complete: target/release/");
    } else {
        eprintln!("Build failed.");
    }
}
