use std::str::FromStr;
use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

impl Sizes {
    fn new(bytes: u64) -> Self {
        Sizes {
            bytes: format!("{} bytes", bytes),
            kilobytes: format!("{:.2} kilobytes", bytes as f64 / 1000.0),
            megabytes: format!("{:.2} megabytes", bytes as f64 / 1_000_000.0),
            gigabytes: format!("{:.2} gigabytes", bytes as f64 / 1_000_000_000.0),
        }
    }
}

fn parse_size(input: &str) -> Result<u64, &'static str> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Invalid input format");
    }

    let size = f64::from_str(parts[0]).map_err(|_| "Invalid number")?;
    let unit = parts[1].to_lowercase();

    let bytes = match unit.as_str() {
        "b" | "bytes" => size,
        "kb" | "kilobytes" => size * 1_000.0,
        "mb" | "megabytes" => size * 1_000_000.0,
        "gb" | "gigabytes" => size * 1_000_000_000.0,
        _ => return Err("Invalid unit"),
    };

    Ok(bytes as u64)
}

// Use this to run the file: cargo run -- "24 mb"
pub fn file_formatter() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run \"<size> <unit>\"");
        return;
    }

    let input = &args[1];
    match parse_size(input) {
        Ok(bytes) => {
            let sizes = Sizes::new(bytes);
            println!("{:?}", sizes);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
