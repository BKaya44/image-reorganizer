# Image Reorganizer

## Overview

This tool is designed to rename image files (specifically .png and .jpg formats) in a specified directory, prefixing them with a given name and indexing them numerically. It is built in Rust and leverages the clap crate for parsing command-line arguments. For those who just want to test the code without building it yourself you can download the latest release.

## Prerequisites

To build and run this tool, you will need:

- Rust programming language setup, including cargo, Rust's package manager and build tool. You can install Rust and cargo by following the instructions at https://www.rust-lang.org/tools/install.

- `clap` crate for command-line argument parsing. This dependency is specified in the `Cargo.toml` file and will be automatically handled by `cargo`.

## Command-Line Arguments

The tool supports the following command-line arguments:

`--name` or `-n`: Required. The prefix name to use when renaming image files. This name will be prepended to a numeric index and the file's original extension.

`--dir` or `-d`: Required. The directory containing the image files you wish to rename. The tool checks for the existence of this directory before proceeding with any operations.

## How to Build

1. Ensure you have Rust and cargo installed on your system.
2. Navigate to the root directory of the project (where the Cargo.toml file is located).
3. Run the following command to build the project:

```sh
cargo build --release
```
This command compiles the project in release mode, optimizing the binary for performance. The compiled binary will be located under target/release/.

## How to Use

After building the project, navigate to the target/release/ directory.

Run the tool using the following syntax:

```sh
./image-reorganizer.exe -n [PREFIX_NAME] -d [DIRECTORY_PATH]
```

Replace [PREFIX_NAME] with the desired prefix for renaming files, and [DIRECTORY_PATH] with the path to the directory containing your images.

The tool will prompt you to confirm the renaming operation. Type `y` and press Enter to proceed, or type `n` to cancel.

```sh
Do you want to rename image files in this directory? (y/N):
```
If confirmed, the tool will rename all .png and .jpg files in the specified directory, prefixing them with the given name and a numeric index.

## Example

Suppose you have a directory named photos containing the files img1.png and pic1.jpg. To rename these files with the prefix holiday-, you would run:

```sh
./image-reorganizer.exe -n holiday -d /path/to/photos
```
After confirming the operation, the files will be renamed to holiday-1.png and holiday-2.jpg, respectively.

## Note

The tool currently only supports .png and .jpg file formats.
Ensure that the specified directory exists and contains image files of the supported formats before running the tool.