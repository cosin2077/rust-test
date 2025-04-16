use serde::Serialize;
use std::collections::{HashMap, HashSet};
// use std::io::{self};
use std::fs::Metadata;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::{self, AsyncReadExt};

#[derive(Serialize, Debug)]
pub struct FileStats {
    pub path: String,
    pub word_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub unique_words: usize,
    pub word_frequency: HashMap<String, usize>,
}

#[derive(Serialize)]
pub struct Analysis {
    pub files: Vec<FileStats>,
    pub total_unique_words: usize,
}

pub async fn collect_paths(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    if Path::new(dir).is_file() {
        paths.push(PathBuf::from(dir));
        return Ok(paths);
    }

    let mut entries = fs::read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            let sub_paths =
                Box::pin(collect_paths(path.to_str().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid path")
                })?))
                .await?;
            paths.extend(sub_paths);
        } else {
            paths.push(path);
        }
    }
    Ok(paths)
}

pub async fn analyze_file(path: &Path) -> io::Result<FileStats> {
    let metadata = fs::metadata(path).await?;
    let is_text = is_text_file(path, &metadata);
    let (content, char_count) = if is_text {
        let mut file = File::open(path).await?;
        let mut content = String::new();
        file.read_to_string(&mut content).await?;
        (content.clone(), content.chars().count())
    } else {
        ("[binary]".to_string(), 0)
    };
    let word_count = if is_text {
        content.split_whitespace().count()
    } else {
        0
    };
    let line_count = if is_text { content.lines().count() } else { 0 };
    let unique_words = if is_text {
        content
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect::<HashSet<String>>()
            .len()
    } else {
        0
    };
    let mut word_frequency = HashMap::new();
    if is_text {
        for word in content.split_whitespace() {
            word_frequency
                .entry(word.to_lowercase())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    Ok(FileStats {
        path: path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid path encoding"))?
            .to_string(),
        word_count,
        line_count,
        char_count,
        unique_words,
        word_frequency,
    })
}
pub async fn analyze_path(path: &str) -> io::Result<Analysis> {
    let paths = collect_paths(path).await?;
    let mut tasks = vec![];
    for p in paths.iter().cloned() {
        let p_owned = p;
        tasks.push(tokio::task::spawn(
            async move { analyze_file(&p_owned).await },
        ));
    }
    let mut files = vec![];
    let mut all_unique = HashSet::new();
    for task in tasks {
        let stats = task.await??;
        all_unique.extend(stats.word_frequency.keys().cloned());
        files.push(stats);
    }
    Ok(Analysis {
        files,
        total_unique_words: all_unique.len(),
    })
}
pub fn find_word(stats: &FileStats, word: &str) -> Option<usize> {
    stats.word_frequency.get(&word.to_lowercase()).copied()
}
fn is_text_file(path: &Path, metadata: &Metadata) -> bool {
    if metadata.len() == 0 {
        return true;
    }
    path.extension()
        .map(|ext| {
            matches!(
                ext.to_str().unwrap_or("").to_lowercase().as_str(),
                "txt"
                    | "json"
                    | "md"
                    | "rs"
                    | "py"
                    | "js"
                    | "html"
                    | "css"
                    | "xml"
                    | "csv"
                    | "toml"
                    | "yaml"
                    | "yml"
            )
        })
        .unwrap_or(false)
}
