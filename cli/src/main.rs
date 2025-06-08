use std::fs;
use std::process::exit;
use std::io::{self, Read};
use clap::{Parser, Subcommand};
use std::path::Path;

/// Axiom CLI: run and check Axiom scripts
#[derive(Parser)]
#[command(name = "axiom")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { file: String },
    Check { file: String },
}

/// Read the core library + user file, concatenate, and return the full source.
fn load_with_core(file: &str) -> String {
    if file == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .unwrap_or_else(|e| { eprintln!("Error reading stdin: {}", e); exit(1) });
        return buf;
    }

    let mut source = String::new();
    let core_path = Path::new("../lib/core.axo");
    if let Ok(lib) = fs::read_to_string(core_path) {
        source.push_str(&lib);
        source.push_str("\n");
    }
    let script = fs::read_to_string(file)
        .unwrap_or_else(|e| { eprintln!("Error reading {}: {}", file, e); exit(1) });
    source.push_str(&script);
    source
}


fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Run { file } => {
            let source = load_with_core(&file);
            match engine::run(&source) {
                Ok(()) => {},
                Err(e) => { eprintln!("Error: {}", e); exit(1) }
            }
        }
        Commands::Check { file } => {
            let source = load_with_core(&file);
            match engine::run(&source) {
                Ok(()) => { println!("OK"); },
                Err(e) => { eprintln!("Error: {}", e); exit(1) }
            }
        }
    }
}
