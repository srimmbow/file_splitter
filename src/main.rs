use num_format::{Locale, ToFormattedString};
use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

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
        let file_result = File::open(file_path);
        if let Err(_e) = file_result {
            eprintln!("Failed to open file: '{}'", file_path);
            continue;
        }
        // No error, file exists
        let mut file = file_result.unwrap();

        let path = Path::new(file_path);
        let file_name = path.file_name().unwrap().to_string_lossy();
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

            fs::write(
                format!("{}/{}.bin", sliced_file_folder_path, i),
                &buf[..bytes_read],
            )?;

            i += 1;
        }

        println!("Successfully sliced: '{}'", file_path);
    }

    Ok(())
}
