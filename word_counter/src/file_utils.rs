use std::collections::HashMap;
use std::fs;

pub fn count_words(file_path: &str) -> Result<usize, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let word_count = content.split_whitespace().count();
    Ok(word_count)
}

pub fn count_lines(file_path: &str) -> Result<usize, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let line_count = content.lines().count();
    Ok(line_count)
}

pub fn count_chars(file_path: &str) -> Result<usize, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("failed to read file: {}", e))?;
    let char_lines = content.chars().count();
    Ok(char_lines)
}

pub fn word_frequency(file_path: &str) -> Result<HashMap<String, usize>, String> {
    let content =
        fs::read_to_string(file_path).map_err(|e| format!("failed to read file: {}", e))?;

    let mut freq = HashMap::new();
    for word in content.split_whitespace() {
        freq.entry(word.to_lowercase())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    Ok(freq)
}
