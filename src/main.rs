use clap::Parser;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn get_size(path: &Path) -> std::io::Result<u64> {
    let metadata = fs::metadata(path)?;
    if metadata.is_file() {
        Ok(metadata.len())
    } else if metadata.is_dir() {
        let mut total_size = 0;
        for entry in WalkDir::new(path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let metadata = entry.metadata()?;
                total_size += metadata.len();
            }
        }
        Ok(total_size)
    } else {
        Ok(0)
    }
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "A Rust CLI utility for getting the size of a file or directory"
)]
struct Args {
    #[arg(index = 1, value_name = "PATH")]
    path: String,
}

fn format_bytes(bytes: u64, path: String) {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    const KIBI: f64 = 1024.0;
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= KIBI && unit_index < UNITS.len() - 1 {
        size /= KIBI;
        unit_index += 1;
    }

    println!("Size \"{}\" is {:.2} {}", path, size, UNITS[unit_index]);
}

fn main() {
    let args = Args::parse();

    match get_size(Path::new(&args.path)) {
        Ok(size) => format_bytes(size, args.path),
        Err(error) => eprintln!("Error getting size: {}", error),
    }
}
