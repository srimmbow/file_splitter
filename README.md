# File Slicer

A simple command-line utility written in Rust that splits files into smaller chunks of a specified size.

## Features

- Split any file into fixed-size chunks
- Process multiple files in a single command
- Automatically creates organized output directories
- Fast and memory-efficient using buffered reading
- Simple `.bin` output format for easy reassembly

## Installation

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Building from Source

```bash
git clone <your-repo-url>
cd file-slicer
cargo build --release
```

The compiled binary will be available at `target/release/file-slicer`.

## Usage

```bash
file-slicer <file_slice_size_in_bytes> <file1_path> [file2_path] [...]
```

### Arguments

- `file_slice_size_in_bytes`: Size of each chunk in bytes (minimum: 1 byte)
- `file_path(s)`: One or more files to split

### Example

Split a large video file into 10MB chunks:

```bash
file-slicer 10485760 video.mp4
```

Split multiple files into 1KB chunks:

```bash
file-slicer 1024 document.pdf image.jpg archive.zip
```

## Output

For each processed file, the program creates a directory named `<filename>_sliced` containing:
- Sequential binary chunks named `0.bin`, `1.bin`, `2.bin`, etc.
- Each chunk is exactly the specified size (except possibly the last chunk)

### Example Output Structure

```
input_file.txt
input_file.txt_sliced/
├── 0.bin
├── 1.bin
├── 2.bin
└── 3.bin
```

## Error Handling

The program will skip files and continue processing if:
- A file cannot be opened
- The output directory already exists (prevents accidental overwrites)
- An invalid file path is provided

## Reassembling Files

To reassemble the sliced files, you can use standard Unix tools:

```bash
cat filename_sliced/*.bin > filename_restored
```

Or on Windows (PowerShell):

```powershell
Get-Content filename_sliced\*.bin -Encoding Byte -Raw | Set-Content filename_restored -Encoding Byte
```

## Dependencies

- `num_format` - For formatted number display

## License

[Add your license here]

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
