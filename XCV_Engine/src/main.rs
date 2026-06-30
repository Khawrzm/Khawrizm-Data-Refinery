use xcv_engine::grid::{XcvSheet, CellValue};
use xcv_engine::ffi;

use std::io::{self, Write};

fn main() {
    println!("--- KHAWRIZM XCV ENGINE v1.0 ---");
    let mut sheet = XcvSheet::new("Live_Sheet");
    let engine = ffi::new_engine();

    loop {
        print!("\nXCV_Core> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0].to_uppercase().as_str() {
            "SET" if parts.len() >= 3 => {
                let cell_id = parts[1];
                let val = if parts[2].starts_with('=') {
                    CellValue::Formula(parts[2..].join(" "))
                } else {
                    CellValue::Text(parts[2..].join(" "))
                };
                sheet.set_cell(cell_id, val);
                println!("[+] Cell {} updated.", cell_id.to_uppercase());
            }
            "EVAL" if parts.len() == 2 => {
                if let CellValue::Formula(f) = sheet.get_cell(parts[1]) {
                    println!("[✓] {} -> {}", f, engine.evaluate_formula(&f, &sheet));
                }
            }
            "EXIT" => break,
            _ => println!("[!] Unknown command."),
        }
    }
}
