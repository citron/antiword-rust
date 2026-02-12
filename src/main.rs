mod doc_parser;
mod parallel_processor;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

use parallel_processor::{find_doc_files, process_multiple_files, process_single_file};

/// A fast, parallelized Microsoft Word (.doc) to text converter
#[derive(Parser, Debug)]
#[command(name = "antiword-rust")]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file or directory containing .doc files
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output directory for converted text files (if not specified, prints to stdout)
    #[arg(short, long, value_name = "DIR")]
    output: Option<PathBuf>,

    /// Process files recursively when input is a directory
    #[arg(short, long)]
    recursive: bool,

    /// Show verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let start_time = Instant::now();

    // Find all .doc files
    let doc_files = if args.input.is_file() {
        vec![args.input.clone()]
    } else if args.recursive || args.input.is_dir() {
        find_doc_files(&args.input)
            .with_context(|| format!("Failed to find .doc files in: {}", args.input.display()))?
    } else {
        vec![args.input.clone()]
    };

    if doc_files.is_empty() {
        eprintln!("No .doc files found in: {}", args.input.display());
        return Ok(());
    }

    if args.verbose {
        eprintln!("Found {} .doc file(s) to process", doc_files.len());
    }

    // Process files
    let results = if doc_files.len() == 1 && args.output.is_none() {
        // Single file to stdout - don't use parallelization
        vec![process_single_file(&doc_files[0], None)?]
    } else {
        // Multiple files or output to directory - use parallel processing
        process_multiple_files(doc_files, args.output.as_deref())
    };

    // Report results
    let successful = results.iter().filter(|r| r.success).count();
    let failed = results.len() - successful;

    if args.verbose || failed > 0 {
        let elapsed = start_time.elapsed();
        eprintln!(
            "\nProcessed {} file(s) in {:.2}s ({} successful, {} failed)",
            results.len(),
            elapsed.as_secs_f64(),
            successful,
            failed
        );

        // Show failed files
        for result in results.iter().filter(|r| !r.success) {
            eprintln!(
                "  ERROR: {} - {}",
                result.input_path.display(),
                result.error.as_deref().unwrap_or("Unknown error")
            );
        }

        // Show successful conversions
        if args.verbose {
            for result in results.iter().filter(|r| r.success) {
                if let Some(output) = &result.output_path {
                    eprintln!(
                        "  OK: {} -> {}",
                        result.input_path.display(),
                        output.display()
                    );
                }
            }
        }
    }

    // Exit with error code if any files failed
    if failed > 0 {
        std::process::exit(1);
    }

    Ok(())
}
