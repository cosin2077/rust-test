use std::fs::Metadata;
use std::path::{Path, PathBuf};

use tokio::fs::{self, File};
use tokio::io::{self, AsyncReadExt};

pub async fn collect_paths(dir: &str) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let mut entries = fs::read_dir(dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            let sub_paths =
                Box::pin(collect_paths(path.to_str().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidData, "Invalid path")
                })?))
                .await?;
            // println!("sub_paths: {:?}", sub_paths);
            paths.extend(sub_paths);
        } else {
            println!("file_paths: {:?}", path);
            paths.push(path);
            // panic!("GOGO")
        }
    }
    Ok(paths)
}

pub async fn read_file_content(path: &Path) -> io::Result<String> {
    let metadata = fs::metadata(path).await?;
    if !is_text_file(path, &metadata) {
        return Ok("[binary]".to_string());
    }

    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

fn is_text_file(path: &Path, metadata: &Metadata) -> bool {
    if metadata.len() == 0 {
        return true; // 空文件视为文本
    }

    path.extension()
        .map(|ext| {
            matches!(
                ext.to_str().unwrap_or("").to_lowercase().as_str(),
                "txt" | "json" | "md" | "rs" | "js" | "html" | "css" | "toml" | "yml"
            )
        })
        .unwrap_or(false)
}
