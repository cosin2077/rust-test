use chrono::Utc;
use clap::Parser;
use file_utils::{collect_paths, read_file_content};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};


#[derive(Parser, Debug)]
struct Args {
    #[clap(long, default_value = ".")]
    dir: String,
    #[clap(long, default_value = "1000")]
    max_line: String,
}

fn parse_max_line(input: &str) -> Option<usize> {
    match input.to_lowercase().as_str() {
        "infinity" | "-1" | "0" => None,
        _ => input.parse::<usize>().ok(),
    }
}

async fn write_output(paths: Vec<&PathBuf>, max_lines: Option<usize>) -> io::Result<()> {
    let timestamp = Utc::now().timestamp_millis();
    let output_file = format!("treesave.{}.txt", timestamp);
    let mut file = File::create(&output_file).await?;
    let mut line_count = 0;

    for path in paths {
        if let Some(max) = max_lines {
            if line_count >= max {
                break;
            }
        }

        let content = read_file_content(path).await?;
        let path_str = path
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid path"))?;

        let header = format!("{}:\n", path_str);
        file.write_all(header.as_bytes()).await?;
        line_count += 1;

        if content != "[binary]" {
            file.write_all(content.as_bytes()).await?;
            let content_lines = content.lines().count();
            line_count += content_lines;
            if content_lines > 0 || content.is_empty() {
                file.write_all(b"\n").await?;
                line_count += 1;
            }
        } else {
            file.write_all(b"[binary]\n\n").await?;
            line_count += 2;
        }
    }

    file.flush().await?;
    println!("Output written to {}", output_file);
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    println!("Raw arguments: {:?}", args);
    let max_lines = parse_max_line(&args.max_line);

    let path_vec = collect_paths(&args.dir).await?;
    let paths = path_vec.iter().filter(|p| p.is_file()).collect::<Vec<_>>();

    if paths.is_empty() {
        println!("No files found in {}", args.dir);
        return Ok(());
    }

    write_output(paths, max_lines).await?;
    Ok(())
}

mod file_utils;
