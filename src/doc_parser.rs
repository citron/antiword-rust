use anyhow::{Context, Result};
use cfb::CompoundFile;
use encoding_rs::WINDOWS_1252;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Extract text from a Microsoft Word .doc file
pub fn extract_text_from_doc<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();
    let file =
        File::open(path).with_context(|| format!("Failed to open file: {}", path.display()))?;

    let mut comp = CompoundFile::open(BufReader::new(file))
        .with_context(|| format!("Failed to parse OLE file: {}", path.display()))?;

    // Try to read the WordDocument stream which contains the main document text
    let mut stream = comp
        .open_stream("WordDocument")
        .or_else(|_| comp.open_stream("worddocument"))
        .context("Failed to find WordDocument stream in .doc file")?;

    let mut buffer = Vec::new();
    stream
        .read_to_end(&mut buffer)
        .context("Failed to read WordDocument stream")?;

    // Extract text from the binary data
    let text = extract_text_from_binary(&buffer);

    Ok(text)
}

/// Extract text from binary Word document data
fn extract_text_from_binary(data: &[u8]) -> String {
    let mut text = String::new();
    let mut i = 0;

    // Skip the FIB (File Information Block) - first 1536 bytes typically
    if data.len() > 1536 {
        i = 1536;
    }

    // Simple text extraction - look for printable characters
    // This is a simplified approach; real .doc parsing is complex
    while i < data.len() {
        let byte = data[i];

        // Check for printable ASCII and common extended ASCII
        if (32..=126).contains(&byte) || byte == b'\n' || byte == b'\r' || byte == b'\t' {
            text.push(byte as char);
        } else if byte >= 128 {
            // Try to decode as Windows-1252 (common in old Word docs)
            if i + 1 < data.len() {
                let slice = &data[i..i + 1];
                let (decoded, _, had_errors) = WINDOWS_1252.decode(slice);
                if !had_errors && !decoded.chars().all(|c| c.is_control()) {
                    text.push_str(&decoded);
                }
            }
        }

        i += 1;
    }

    // Clean up the text
    cleanup_text(&text)
}

/// Clean up extracted text by removing excessive whitespace and control characters
fn cleanup_text(text: &str) -> String {
    let mut result = String::new();
    let mut prev_was_space = false;

    for c in text.chars() {
        if c.is_whitespace() {
            if !prev_was_space {
                if c == '\n' || c == '\r' {
                    result.push('\n');
                } else {
                    result.push(' ');
                }
                prev_was_space = true;
            }
        } else if c.is_control() {
            // Skip other control characters
            continue;
        } else {
            result.push(c);
            prev_was_space = false;
        }
    }

    // Trim and remove multiple consecutive newlines
    result
        .split('\n')
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_text() {
        let input = "Hello    world\n\n\nTest    text  \n";
        let output = cleanup_text(input);
        assert_eq!(output, "Hello world\nTest text");
    }

    #[test]
    fn test_cleanup_text_with_controls() {
        let input = "Hello\x00\x01world";
        let output = cleanup_text(input);
        assert_eq!(output, "Helloworld");
    }
}
