#![allow(unused)]
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use serde::Deserialize;
use serde_json::json;
use ureq;

#[derive(Deserialize, Debug, Default)]
struct Structured {
    #[serde(default)]
    section_title: String,
    #[serde(default)]
    executive_summary: String,
    #[serde(default)]
    entities: Vec<String>,
    #[serde(default)]
    structured_markdown: String,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct OllamaMessage { content: String }
#[derive(Deserialize, Debug)]
struct OllamaResp { message: OllamaMessage }

fn chunk_text(text: &str, max: usize) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut cur = String::new();
    for para in text.split("\n\n") {
        if cur.len() + para.len() + 2 > max && !cur.is_empty() {
            chunks.push(cur.trim().to_string());
            cur = para.to_string();
        } else {
            if !cur.is_empty() { cur.push_str("\n\n"); }
            cur.push_str(para);
        }
    }
    if !cur.is_empty() { chunks.push(cur.trim().to_string()); }
    if chunks.is_empty() { chunks.push(text.to_string()); }
    chunks
}

fn sanitize_chunk(chunk: &str) -> Option<Structured> {
    let system = "You are Ring-0 Data Structurer. Output ONLY valid JSON matching the schema. No other text. Schema: {section_title, executive_summary, entities: string[], structured_markdown, tags: string[]}";
    let payload = json!({
        "model": "llama3",
        "messages": [ {"role": "system", "content": system}, {"role": "user", "content": format!("RAW DATA:\n{}", chunk)} ],
        "stream": false,
        "format": "json",
        "options": {"temperature": 0.0}
    });
    let resp: OllamaResp = match ureq::post("http://127.0.0.1:11434/api/chat")
        .set("Content-Type", "application/json")
        .send_json(payload) {
            Ok(r) => match r.into_json() { Ok(v) => v, _ => return None },
            _ => return None,
        };
    let content = resp.message.content;
    let parsed: Structured = match serde_json::from_str(&content) {
        Ok(p) if !p.structured_markdown.is_empty() => p,
        _ => return None,
    };
    Some(parsed)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "-" };
    let output = if args.len() > 2 { &args[2] } else { "Master_Ring0.md" };
    let text = if input == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap_or_default();
        buf
    } else {
        fs::read_to_string(input).unwrap_or_default()
    };
    if text.trim().is_empty() { return; }
    let chunks = chunk_text(&text, 3200);
    let mut final_md = String::from("# Ring-0 Master Synthesized Corpus (v1.4 Rust)\n\n---\n\n");
    for (i, ch) in chunks.iter().enumerate() {
        eprintln!("[Ring-0] Sanitizing chunk {}/{}", i+1, chunks.len());
        if let Some(s) = sanitize_chunk(ch) {
            final_md.push_str(&format!("## {}\n\n**Summary:** {}\n\n**Entities:** {}\n\n{}\n\n**Tags:** {}\n\n---\n\n",
                s.section_title, s.executive_summary, s.entities.join(", "), s.structured_markdown, s.tags.join(", ")));
        }
    }
    let out_path = Path::new(output);
    let mut f = OpenOptions::new().create(true).write(true).truncate(true).open(out_path).unwrap();
    f.write_all(final_md.as_bytes()).unwrap();
    eprintln!("[Ring-0] Wrote {} bytes to {}", final_md.len(), output);
}
