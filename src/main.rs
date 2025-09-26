use clap::Parser; 
use std::fs;
use std::path::Path;

fn get_size(path: &Path) -> std::io::Result<u64>{
    let metadata = fs::metadata(path)?;
    if metadata.is_file() {
        Ok(metadata.len())
    }
    //else if metadata.is_dir() {
        //
    //}
    else {
        Ok(0)
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'p', long = "path", value_name = "PATH")]
    path: String,
}

fn main() {
    let args = Args::parse();
    match get_size(Path::new(&args.path)) {
        Ok(size) => println!("{}", size),
        Err(error) => eprintln!("Ошибка при получении размера: {}", error), // Печатаем ошибку в stderr
    }
}
