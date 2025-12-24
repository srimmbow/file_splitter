# File Slicer

A lightweight Rust utility for splitting large files into smaller chunks and reassembling them.

## Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs/)

### Build

```bash
git clone <your-repo-url>
cd file-slicer
cargo build --release
```

Binaries will be created in `target/release/`:
- `file-slicer` - splits files into chunks
- `file-assembler` - reassembles chunks back into original files

## Usage

### Slice a File

```bash
file-slicer <chunk_size_bytes> <file> [more_files...]
```

**Example:** Split `video.mp4` into 10MB chunks
```bash
file-slicer 10485760 video.mp4
```

This creates a folder `video.mp4_sliced/` containing:
```
video.mp4_sliced/
├── 0.bin
├── 1.bin
├── 2.bin
└── ...
```

### Reassemble a File

```bash
file-assembler <sliced_folder> <output_file>
```

**Example:** Reassemble the chunks
```bash
file-assembler video.mp4_sliced video_restored.mp4
```

## Features

- Fast buffered I/O for efficient processing
- Process multiple files at once
- Automatically skips existing output folders (prevents overwrites)
- Simple numbered `.bin` format for easy verification

## Dependencies

- `num-format` - Number formatting for error messages

## License

[Add your license here]