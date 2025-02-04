use clap::Parser;
use std::path::PathBuf;
use std::fs::{read_dir, read_to_string, File, OpenOptions};
use std::io::Write;

#[derive(Parser)]
#[command(name = "tloc")]
#[command(about = "a simple solidity codebase parser")]
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

    let mut total_code_count = 0;

    let mut file = OpenOptions::new()
        .write(true)        // Enable append mode
        .create(true)       // Create the file if it doesn't exist
        .truncate(true)     // Wipe file if already exists
        .open(&args.out).unwrap();     // Open the file, returning a Result, unwrap it
    
    writeln!(file, "# Scope").unwrap();

    // Handle user's path is a specific file location
    // Handle user's path is a directory
    // Handle user error where user passed neither
    if args.path.is_file() {
        if let Some(extension) = args.path.extension() {
            if extension == "sol" {
                total_code_count = write_output(&mut file, &args.path);
            }
        }       
    } else if args.path.is_dir() {
        total_code_count = traverse_dir(&args.path, &mut file, &args.recurse);
    } else {
        panic!("PATH READ ERROR");
    }

    writeln!(file, "TOTAL: {}", total_code_count).unwrap();
}

pub fn traverse_dir(path: &PathBuf, file :&mut File, traverse: &bool) -> u32 {
    let mut code_count = 0;
    // Loop over directory
    for entry in read_dir(path).expect("failed to read path") {
        let entry = entry.expect("failed to handle file");
        let path = entry.path();

        // If file calc SLOC & write to file
        // If directory traverse directory (IF traverse == true)
        if path.is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "sol" {  // For Solidity files
                    // Process the file
                    code_count += write_output(file, &entry.path());                
                }
            }
        } else if path.is_dir() {
            if *traverse {
                let dir_name = &path.file_name().unwrap().to_string_lossy();
                writeln!(file, "## {}", dir_name).unwrap();
                code_count += traverse_dir(&path, file, &true);
            }
        }
    }
    code_count
}

pub fn write_output(file: &mut File, entry: &PathBuf) -> u32 {
    let content = read_to_string(entry).unwrap();
    let mut code_count = 0;

    let mut multi_line_comment = false;
    for line in content.lines() {
        // Handle being inside a multiline comment
        if multi_line_comment {
            if line.contains("*/") {
                multi_line_comment = false;
            }
            continue;
        } 
        match line.trim() {
            val if val.len() == 0 => continue,
            val if val.starts_with("//") => continue,
            val if val.starts_with("/*") => {
                multi_line_comment = true;
            },
            _ => code_count += 1,
        }
    }

    // Write - `FileName.sol` (COUNT) to file
    let file_name = &entry.file_name().unwrap().to_string_lossy();
    let output = format!("- `{}` ({})", &file_name, code_count);
    writeln!(file, "{}", output).unwrap();

    code_count
}

