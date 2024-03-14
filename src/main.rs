use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    dir: String,
}

fn confirm_action(prompt: &str) -> io::Result<bool> {
    let mut input = String::new();
    print!("{} (y/N): ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}

fn main() {
    let args = Args::parse();

    if Path::new(&args.dir).exists() {
        println!("Directory '{}' found.", &args.dir);

        if let Ok(confirm) = confirm_action("Do you want to rename image files in this directory?") {
            if confirm {
                match fs::read_dir(&args.dir) {
                    Ok(entries) => {
                        let mut count = 1;
                        for entry in entries.filter_map(Result::ok) {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
                                    if extension == "png" || extension == "jpg" {
                                        let new_name = format!("{}-{}.{}", args.name, count, extension);
                                        let new_path = PathBuf::from(&args.dir).join(new_name);
                                        if let Err(e) = fs::rename(&path, &new_path) {
                                            println!("Error renaming file '{:?}' to '{:?}': {}", path, new_path, e);
                                        } else {
                                            println!("Renamed '{:?}' to '{}'", path, new_path.display());
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => println!("Failed to read directory '{}': {}", &args.dir, e),
                }
            } else {
                println!("Operation canceled.");
            }
        }
    } else {
        println!("Directory '{}' does not exist.", &args.dir);
    }
}
