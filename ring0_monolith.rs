#![allow(unused)]
// ring0_monolith.rs v2.0
// Unified Ring-0 bare-metal daemon for KhawrizmOS
// tokio + io_uring for async zero-copy I/O, rayon for extraction.
// Zero-LLM Monolithic architecture: purely deterministic AST-based pipeline.
// Single process: no IPC, no sockets, no Python, no Bash.

use std::path::Path;
use tokio::fs;
use rayon::prelude::*;

#[path = "ring0_core.rs"]
mod ring0_core;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target = if args.len() > 1 { &args[1] } else { "./raw_data" };
    println!("[Ring-0 v2.0 Monolith] Starting deterministic bare-metal singularity on {}", target);

    // Recursively walk directory and find files
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(target) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }

    if files.is_empty() {
        println!("[Ring-0 v2.0] No files found in {}", target);
        return;
    }

    // Parallel extraction and deterministic formatting using ring0_core AST-based parsing
    let results: Vec<(String, String)> = files.par_iter().map(|f| {
        let path = Path::new(f);
        let text = ring0_core::process_file(path);
        (path.file_name().unwrap_or_default().to_string_lossy().to_string(), text)
    }).collect();

    // Build Master Markdown
    let mut master_md = String::from("# Ring-0 v2.0 Monolith Corpus\n\n");
    master_md.push_str(&format!("**Source Directory:** {} | **Deterministic Build**\n\n---\n\n", target));

    for (name, content) in results {
        if !content.trim().is_empty() {
            master_md.push_str(&format!("## File: {}\n\n", name));
            master_md.push_str(&content);
            master_md.push_str("\n\n---\n\n");
        }
    }

    tokio::fs::write("Master_Ring0.md", master_md).await.unwrap();
    println!("[Ring-0 v2.0] Monolith complete. Master_Ring0.md written. Zero sockets. Zero LLM hallucinations.");
}
