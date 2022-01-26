use std::fs;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author = "Axel Kappel", version, about = "A utility to clean filenames", long_about = None)]
struct Args {
    pattern: String,
    #[clap(short = 'r', long)]
    replace: Option<String>,
    #[clap(short = 'f', long)]
    force: bool,
    #[clap(short = 'd', long)]
    directory: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let dir = fs::read_dir(args.directory.as_ref().map(|s| s.as_str()).unwrap_or("."))?;
    let mut renames = Vec::new();

    let replace = args.replace.as_ref().map(|s| s.as_str()).unwrap_or("");

    for entry in dir {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            let path = entry.path();
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            if stem.contains(&args.pattern) {
                if let Some(ext) = path.extension() {
                    let extension = ext.to_string_lossy();
                    let new_stem = stem.replace(&args.pattern, replace);
                    let to = format!("{}.{}", new_stem, extension);
                    renames.push((path.to_string_lossy().to_string(), to));
                }
            }
        }
    }

    renames.sort();
    for (from, to) in &renames {
        println!("'{}' -> '{}'", from, to);
    }

    if renames.is_empty() {
        eprintln!("No files with '{}' in the name found", &args.pattern);
        std::process::exit(1);
    } else {
        let clean = if args.force {
            true
        } else {
            dialoguer::Confirm::new()
                .with_prompt("Are you sure you want to rename these files?")
                .wait_for_newline(true)
                .interact()?
        };

        if clean {
            for (from, to) in &renames {
                fs::rename(from, to)?;
            }
        }
    }

    Ok(())
}
