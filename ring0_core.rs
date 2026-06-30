#![allow(unused)]
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;
use memmap2::Mmap;
use rayon::prelude::*;
use regex::Regex;
use flate2::read::ZlibDecoder;
use zip::ZipArchive;
use quick_xml::events::Event;
use quick_xml::Reader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ring0_core <file|directory>");
        std::process::exit(1);
    }
    let p = Path::new(&args[1]);
    if p.is_dir() {
        process_directory_parallel(p);
    } else if p.is_file() {
        let text = process_file(p);
        println!("{}", text);
    } else {
        eprintln!("Not found");
        std::process::exit(1);
    }
}

pub fn process_file(path: &Path) -> String {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
    let text = match ext.as_str() {
        "pdf" => extract_pdf_mmap(path),
        "docx" | "doc" => extract_docx(path),
        "xlsx" | "xls" => extract_xlsx(path),
        "pptx" => extract_pptx(path),
        "html" | "htm" => extract_html(path),
        _ => extract_plain_mmap(path),
    };
    let t = clean_terminal_noise(&text);
    let t = fix_pdf_kerning(&t);
    fix_arabic_reversal(&t)
}

fn read_mmap(path: &Path) -> io::Result<Mmap> {
    let file = File::open(path)?;
    unsafe { Mmap::map(&file) }
}

fn extract_plain_mmap(path: &Path) -> String {
    if let Ok(mmap) = read_mmap(path) {
        String::from_utf8_lossy(&mmap).to_string()
    } else {
        std::fs::read_to_string(path).unwrap_or_default()
    }
}

fn extract_pdf_mmap(path: &Path) -> String {
    let data = if let Ok(mmap) = read_mmap(path) { mmap.to_vec() } else { std::fs::read(path).unwrap_or_default() };
    let mut text_parts = Vec::new();
    
    // Scan manually for streams
    let mut search_idx = 0;
    while let Some(idx) = data[search_idx..].windows(6).position(|w| w == b"stream") {
        let abs_stream_idx = search_idx + idx;
        search_idx = abs_stream_idx + 6;
        
        // Determine the start of the stream data
        let mut start_offset = 6;
        if abs_stream_idx + start_offset < data.len() && data[abs_stream_idx + start_offset] == b'\r' {
            start_offset += 1;
        }
        if abs_stream_idx + start_offset < data.len() && data[abs_stream_idx + start_offset] == b'\n' {
            start_offset += 1;
        }
        let stream_start = abs_stream_idx + start_offset;
        
        // Find the matching endstream
        if let Some(end_offset) = data[stream_start..].windows(9).position(|w| w == b"endstream") {
            let abs_end_idx = stream_start + end_offset;
            
            // Extract the stream content slice
            let mut stream_content = &data[stream_start..abs_end_idx];
            
            // Trim trailing \r or \n from stream content
            while !stream_content.is_empty() && (stream_content[stream_content.len() - 1] == b'\n' || stream_content[stream_content.len() - 1] == b'\r') {
                stream_content = &stream_content[..stream_content.len() - 1];
            }
            
            // Decompress
            let mut decoder = ZlibDecoder::new(stream_content);
            let mut dec = Vec::new();
            if decoder.read_to_end(&mut dec).is_ok() {
                let ds = String::from_utf8_lossy(&dec);
                
                // TJ array matching
                let tj_array_re = Regex::new(r"\[([^\]]*)\]\s*TJ").unwrap();
                let inner_re = Regex::new(r"\(([^)]*)\)").unwrap();
                for cap_arr in tj_array_re.captures_iter(&ds) {
                    if let Some(arr) = cap_arr.get(1) {
                        for cap_inner in inner_re.captures_iter(arr.as_str()) {
                            if let Some(t) = cap_inner.get(1) {
                                text_parts.push(t.as_str().to_string());
                            }
                        }
                    }
                }
                
                // Tj matching
                let tj_re = Regex::new(r"\(([^)]*)\)\s*Tj").unwrap();
                for c in tj_re.captures_iter(&ds) {
                    if let Some(t) = c.get(1) {
                        text_parts.push(t.as_str().to_string());
                    }
                }
            }
        }
    }
    
    // Global fallback if no text extracted from streams
    if text_parts.is_empty() {
        let raw_str = String::from_utf8_lossy(&data);
        let tj_re = Regex::new(r"\(([^)]{3,})\)\s*Tj").unwrap();
        for c in tj_re.captures_iter(&raw_str) {
            if let Some(t) = c.get(1) {
                text_parts.push(t.as_str().to_string());
            }
        }
    }
    
    text_parts.join(" ")
}

fn extract_docx(path: &Path) -> String {
    let Ok(f) = File::open(path) else { return String::new(); };
    let Ok(mut archive) = ZipArchive::new(f) else { return String::new(); };
    let mut xml = String::new();
    if archive.by_name("word/document.xml").ok().map(|mut d| d.read_to_string(&mut xml)).is_none() {
        return String::new();
    }

    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut in_tbl = false;
    let mut in_tr = false;
    let mut in_tc = false;
    let mut in_p = false;
    let mut in_t = false;
    let mut heading_level = None;
    let mut current_text = String::new();
    let mut current_row = Vec::new();
    let mut current_table = Vec::new();
    let mut out = String::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name().as_ref() {
                    b"w:tbl" => {
                        in_tbl = true;
                        current_table = Vec::new();
                    }
                    b"w:tr" => {
                        in_tr = true;
                        current_row = Vec::new();
                    }
                    b"w:tc" => {
                        in_tc = true;
                        current_text = String::new();
                    }
                    b"w:p" => {
                        in_p = true;
                        if !in_tc {
                            current_text = String::new();
                            heading_level = None;
                        }
                    }
                    b"w:pStyle" => {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"w:val" {
                                let val = String::from_utf8_lossy(&attr.value);
                                if val.contains("Heading1") {
                                    heading_level = Some(1);
                                } else if val.contains("Heading2") {
                                    heading_level = Some(2);
                                } else if val.contains("Heading3") {
                                    heading_level = Some(3);
                                }
                            }
                        }
                    }
                    b"w:t" => {
                        in_t = true;
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                match e.name().as_ref() {
                    b"w:tbl" => {
                        in_tbl = false;
                        if !current_table.is_empty() {
                            let max_cols = current_table.iter().map(|r: &Vec<String>| r.len()).max().unwrap_or(0);
                            for (r_idx, row) in current_table.iter().enumerate() {
                                let mut row_cells = row.clone();
                                while row_cells.len() < max_cols {
                                    row_cells.push(String::new());
                                }
                                out.push_str(&format!("| {} |\n", row_cells.join(" | ")));
                                if r_idx == 0 {
                                    let sep: Vec<String> = (0..max_cols).map(|_| "---".to_string()).collect();
                                    out.push_str(&format!("| {} |\n", sep.join(" | ")));
                                }
                            }
                            out.push_str("\n");
                        }
                    }
                    b"w:tr" => {
                        in_tr = false;
                        current_table.push(current_row.clone());
                    }
                    b"w:tc" => {
                        in_tc = false;
                        current_row.push(current_text.clone());
                    }
                    b"w:p" => {
                        in_p = false;
                        if !in_tc {
                            let trimmed = current_text.trim().to_string();
                            if !trimmed.is_empty() {
                                if let Some(level) = heading_level {
                                    out.push_str(&format!("{} {}\n\n", "#".repeat(level), trimmed));
                                } else {
                                    out.push_str(&format!("{}\n\n", trimmed));
                                }
                            }
                        }
                    }
                    b"w:t" => {
                        in_t = false;
                    }
                    _ => {}
                }
            }
            Ok(Event::Text(e)) if in_t => {
                if let Ok(text) = e.unescape() {
                    if in_tc || in_p {
                        current_text.push_str(&text);
                    }
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
        buf.clear();
    }
    out
}

fn col_to_index(col: &str) -> usize {
    let mut index = 0;
    for c in col.chars() {
        if c.is_ascii_alphabetic() {
            index = index * 26 + (c.to_ascii_uppercase() as usize - 'A' as usize + 1);
        }
    }
    if index > 0 { index - 1 } else { 0 }
}

fn parse_cell_ref(r: &str) -> (usize, usize) {
    let col_part: String = r.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    let row_part: String = r.chars().skip_while(|c| c.is_ascii_alphabetic()).collect();
    let col = col_to_index(&col_part);
    let row = row_part.parse::<usize>().unwrap_or(1) - 1;
    (col, row)
}

fn extract_xlsx(path: &Path) -> String {
    let Ok(f) = File::open(path) else { return String::new(); };
    let Ok(mut archive) = ZipArchive::new(f) else { return String::new(); };

    // 1. Read shared strings
    let mut shared_strings = Vec::new();
    let mut xml = String::new();
    if archive.by_name("xl/sharedStrings.xml").ok().map(|mut d| d.read_to_string(&mut xml)).is_some() {
        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);
        let mut in_t = false;
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() == b"t" => {
                    in_t = true;
                }
                Ok(Event::End(ref e)) if e.name().as_ref() == b"t" => {
                    in_t = false;
                }
                Ok(Event::Text(e)) if in_t => {
                    if let Ok(text) = e.unescape() {
                        shared_strings.push(text.into_owned());
                    }
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }
    }

    // 2. Read sheets
    let mut out = String::new();
    let file_names: Vec<String> = archive.file_names().map(|n| n.to_string()).collect();
    for name in file_names {
        if name.starts_with("xl/worksheets/sheet") && name.ends_with(".xml") {
            let mut sheet_xml = String::new();
            if archive.by_name(&name).ok().map(|mut d| d.read_to_string(&mut sheet_xml)).is_none() {
                continue;
            }
            
            out.push_str(&format!("### Sheet: {}\n\n", name.trim_start_matches("xl/worksheets/").trim_end_matches(".xml")));

            let mut reader = Reader::from_str(&sheet_xml);
            reader.trim_text(true);
            let mut rows_map: std::collections::BTreeMap<usize, std::collections::BTreeMap<usize, String>> = std::collections::BTreeMap::new();
            let mut in_v = false;
            let mut is_shared_string = false;
            let mut current_col = 0;
            let mut current_row = 0;
            let mut buf = Vec::new();

            loop {
                match reader.read_event_into(&mut buf) {
                    Ok(Event::Start(ref e)) => {
                        match e.name().as_ref() {
                            b"c" => {
                                is_shared_string = false;
                                let mut cell_ref = String::new();
                                for attr in e.attributes().flatten() {
                                    if attr.key.as_ref() == b"t" && attr.value.as_ref() == b"s" {
                                        is_shared_string = true;
                                    } else if attr.key.as_ref() == b"r" {
                                        cell_ref = String::from_utf8_lossy(&attr.value).into_owned();
                                    }
                                }
                                if !cell_ref.is_empty() {
                                    let (col, row) = parse_cell_ref(&cell_ref);
                                    current_col = col;
                                    current_row = row;
                                }
                            }
                            b"v" => {
                                in_v = true;
                            }
                            _ => {}
                        }
                    }
                    Ok(Event::End(ref e)) => {
                        if e.name().as_ref() == b"v" {
                            in_v = false;
                        }
                    }
                    Ok(Event::Text(e)) if in_v => {
                        if let Ok(val_str) = e.unescape() {
                            let mut value = val_str.into_owned();
                            if is_shared_string {
                                if let Ok(idx) = value.parse::<usize>() {
                                    if idx < shared_strings.len() {
                                        value = shared_strings[idx].clone();
                                    }
                                }
                            }
                            rows_map.entry(current_row)
                                .or_default()
                                .insert(current_col, value);
                        }
                    }
                    Ok(Event::Eof) => break,
                    _ => {}
                }
                buf.clear();
            }

            let max_col = rows_map.values()
                .flat_map(|row| row.keys())
                .max()
                .cloned()
                .unwrap_or(0);

            for (row_idx, col_map) in &rows_map {
                let mut row_cells = Vec::new();
                for col_idx in 0..=max_col {
                    let cell_val = col_map.get(&col_idx).cloned().unwrap_or_default();
                    row_cells.push(cell_val);
                }
                out.push_str(&format!("| {} |\n", row_cells.join(" | ")));
                if *row_idx == 0 {
                    let sep: Vec<String> = (0..=max_col).map(|_| "---".to_string()).collect();
                    out.push_str(&format!("| {} |\n", sep.join(" | ")));
                }
            }
            out.push_str("\n\n");
        }
    }
    out
}

fn extract_pptx(path: &Path) -> String {
    let Ok(f) = File::open(path) else { return String::new(); };
    let Ok(mut archive) = ZipArchive::new(f) else { return String::new(); };

    let mut out = String::new();
    let file_names: Vec<String> = archive.file_names().map(|n| n.to_string()).collect();
    let mut slide_names: Vec<String> = file_names.into_iter()
        .filter(|name| name.starts_with("ppt/slides/slide") && name.ends_with(".xml"))
        .collect();
    slide_names.sort_by_key(|name| {
        let num_str: String = name.chars().filter(|c| c.is_ascii_digit()).collect();
        num_str.parse::<usize>().unwrap_or(0)
    });

    for name in slide_names {
        let mut slide_xml = String::new();
        if archive.by_name(&name).ok().map(|mut d| d.read_to_string(&mut slide_xml)).is_none() {
            continue;
        }

        out.push_str(&format!("### Slide: {}\n\n", name.trim_start_matches("ppt/slides/").trim_end_matches(".xml")));

        let mut reader = Reader::from_str(&slide_xml);
        reader.trim_text(true);
        let mut in_p = false;
        let mut in_t = false;
        let mut current_paragraph = String::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    if e.name().as_ref() == b"a:p" {
                        in_p = true;
                        current_paragraph = String::new();
                    } else if e.name().as_ref() == b"a:t" {
                        in_t = true;
                    }
                }
                Ok(Event::End(ref e)) => {
                    if e.name().as_ref() == b"a:p" {
                        in_p = false;
                        let trimmed = current_paragraph.trim();
                        if !trimmed.is_empty() {
                            out.push_str(&format!("- {}\n", trimmed));
                        }
                    } else if e.name().as_ref() == b"a:t" {
                        in_t = false;
                    }
                }
                Ok(Event::Text(e)) if in_t && in_p => {
                    if let Ok(text) = e.unescape() {
                        current_paragraph.push_str(&text);
                    }
                }
                Ok(Event::Eof) => break,
                _ => {}
            }
            buf.clear();
        }
        out.push_str("\n");
    }
    out
}

fn extract_html(path: &Path) -> String {
    let c = std::fs::read_to_string(path).unwrap_or_default();
    Regex::new(r"<[^>]+>").unwrap().replace_all(&c, " ").to_string()
}

fn clean_terminal_noise(t: &str) -> String {
    let re = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    let t = re.replace_all(t, "");
    let re = Regex::new(r"(?m)^\s*(?:kali㎟\S*\s*|PS .*?>\s*|C:\\.*?>|\$\s*|#\s*)\s*").unwrap();
    let t = re.replace_all(&t, "");
    Regex::new(r"\[\d{4}-\d{2}-\d{2}[ T]\d{2}:\d{2}(:\d{2})?\]").unwrap().replace_all(&t, "").to_string().trim().to_string()
}

fn fix_pdf_kerning(t: &str) -> String {
    let mut s = t.to_string();
    for _ in 0..6 { s = Regex::new(r"(\b\w)\s+(\w\b)").unwrap().replace_all(&s, "$1$2").to_string(); }
    s
}

fn fix_arabic_reversal(t: &str) -> String {
    let re = Regex::new(r"[\u0600-\u06FF\u0750-\u077F\u08A0-\u08FF\uFB50-\uFDFF\uFE70-\uFEFF]+").unwrap();
    re.replace_all(t, |c: &regex::Captures| c.get(0).unwrap().as_str().chars().rev().collect::<String>()).to_string()
}

fn process_directory_parallel(dir: &Path) {
    let files: Vec<_> = std::fs::read_dir(dir).unwrap().filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_file()).collect();
    let results: Vec<String> = files.par_iter().map(|f| {
        let t = process_file(f);
        format!("=== BEGIN {}\n{}\n=== END ===\n", f.display(), t)
    }).collect();
    for r in results { print!("{}", r); }
}
