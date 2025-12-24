use num_format::{Locale, ToFormattedString};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};

type FileSize = usize;

const MIN_ARGS: usize = 3;
const MIN_FILE_SLICE_SIZE: FileSize = 1; // in bytes

fn parse_size(s: &str) -> Result<FileSize, String> {
    let size: FileSize = s
        .parse()
        .map_err(|_| format!("'{}' is not a valid file size", s))?;

    if size < MIN_FILE_SLICE_SIZE {
        return Err(format!(
            "File slice size must be greater than {} bytes",
            MIN_FILE_SLICE_SIZE.to_formatted_string(&Locale::en)
        ));
    }

    Ok(size)
}

fn main() -> io::Result<()> {
    // Get command line args
    let args: Vec<String> = env::args().collect();

    // Ensure correct number of arguments
    if args.len() < MIN_ARGS {
        eprintln!(
            "Usage: {} <file_slice_size_in_bytes> <file1_path> <file2_path> ...",
            args[0]
        );
        std::process::exit(1);
    }

    // Get size each file slice should be
    let file_slice_size = match parse_size(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Start reading files, starting from args[2]
    for file_path in &args[2..] {
        let mut file = match File::open(file_path) {
            Ok(f) => BufReader::new(f),
            Err(_) => {
                eprintln!("Failed to open file: '{}'", file_path);
                continue;
            }
        };

        let path = Path::new(file_path);
        let file_name = match path.file_name() {
            Some(name) => name.to_string_lossy(),
            None => {
                eprintln!("Invalid file path: '{}'", file_path);
                continue;
            }
        };
        let sliced_file_folder_path = format!("{}_sliced", file_name);

        let dir_path = Path::new(&sliced_file_folder_path);
        if dir_path.exists() {
            eprintln!(
                "Path '{}' already exists, skipping. Please remove folder to create a new one.",
                sliced_file_folder_path
            );
            continue;
        } else {
            fs::create_dir_all(&sliced_file_folder_path)?;
        }

        // Create buffer and write
        let mut buf = vec![0u8; file_slice_size];
        let mut i = 0;

        loop {
            let bytes_read = file.read(&mut buf)?;
            if bytes_read == 0 {
                break;
            }

            let mut out_path = PathBuf::from(&sliced_file_folder_path);
            out_path.push(format!("{}.bin", i));

            fs::write(out_path, &buf[..bytes_read])?;

            i += 1;
        }

        println!("Successfully sliced: '{}'", file_path);
    }

    Ok(())
}

#[test]
fn parse_size_ok() {
    assert_eq!(parse_size("1022").unwrap(), 1022);
    assert_eq!(parse_size("2222").unwrap(), 2222);
    assert_eq!(parse_size("123123123").unwrap(), 123123123);
}
