use anyhow::{Context, Result};
use rayon::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::doc_parser::extract_text_from_doc;

/// Result of processing a single DOC file
#[derive(Debug)]
pub struct ConversionResult {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub success: bool,
    pub error: Option<String>,
}

/// Process a single DOC file and save the output
pub fn process_single_file(input: &Path, output_dir: Option<&Path>) -> Result<ConversionResult> {
    let input_path = input.to_path_buf();

    match extract_text_from_doc(input) {
        Ok(text) => {
            let output_path = if let Some(out_dir) = output_dir {
                let file_stem = input.file_stem().context("Invalid input filename")?;
                let output = out_dir.join(format!("{}.txt", file_stem.to_string_lossy()));

                fs::write(&output, text).with_context(|| {
                    format!("Failed to write output file: {}", output.display())
                })?;

                Some(output)
            } else {
                // Print to stdout if no output directory specified
                println!("{}", text);
                None
            };

            Ok(ConversionResult {
                input_path,
                output_path,
                success: true,
                error: None,
            })
        }
        Err(e) => Ok(ConversionResult {
            input_path,
            output_path: None,
            success: false,
            error: Some(e.to_string()),
        }),
    }
}

/// Process multiple DOC files in parallel
pub fn process_multiple_files(
    inputs: Vec<PathBuf>,
    output_dir: Option<&Path>,
) -> Vec<ConversionResult> {
    // If output directory is specified, create it
    if let Some(dir) = output_dir {
        fs::create_dir_all(dir).ok();
    }

    // Use rayon for parallel processing
    inputs
        .par_iter()
        .map(|input| {
            process_single_file(input, output_dir).unwrap_or_else(|e| ConversionResult {
                input_path: input.clone(),
                output_path: None,
                success: false,
                error: Some(e.to_string()),
            })
        })
        .collect()
}

/// Find all .doc files in a directory (recursively)
pub fn find_doc_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let path = path.as_ref();

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }

    let mut doc_files = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("doc") {
                    doc_files.push(path.to_path_buf());
                }
            }
        }
    }

    Ok(doc_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_doc_files_empty_dir() {
        let temp_dir = TempDir::new().unwrap();
        let files = find_doc_files(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_find_doc_files_with_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create some test files
        fs::write(temp_dir.path().join("test1.doc"), b"dummy").unwrap();
        fs::write(temp_dir.path().join("test2.txt"), b"dummy").unwrap();
        fs::write(temp_dir.path().join("test3.DOC"), b"dummy").unwrap();

        let files = find_doc_files(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_conversion_result_failure() {
        let result = ConversionResult {
            input_path: PathBuf::from("test.doc"),
            output_path: None,
            success: false,
            error: Some("Test error".to_string()),
        };

        assert!(!result.success);
        assert!(result.error.is_some());
    }
}
