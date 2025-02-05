use clap::Parser;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

use raloc::traverse_dir;

#[derive(Parser)]
#[command(name = "raloc")]
#[command(about = "Rust actual lines of code (minus tests)")]
struct Args {
    #[arg(long, short)]
    path: PathBuf,
    #[arg(long, short)]
    out: PathBuf,
    #[arg(long, short)]
    recurse: bool,
}

fn main() {
    let args = Args::parse();

    let mut file = OpenOptions::new()
        .write(true)        // Enable append mode
        .create(true)       // Create the file if it doesn't exist
        .truncate(true)     // Wipe file if already exists
        .open(&args.out).unwrap();     // Open the file, returning a Result, unwrap it
    
    writeln!(file, "# Scope").unwrap();

    let path = args.path;
    if !path.is_file() && !path.is_dir() {
        panic!("Incorrect Path Provided");
    }

    let total_code_count = traverse_dir(&path, &mut file, &args.recurse, 2);

    writeln!(file, "TOTAL: {}", total_code_count).unwrap();
}

