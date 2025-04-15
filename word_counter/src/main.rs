use clap::Parser;
mod file_utils;
mod utils;
use file_utils::{count_chars, count_lines, count_words, word_frequency};
use utils::*;

#[derive(Parser, Debug)] // Derive Parser for argument parsing
struct Args {
    #[arg(short, long)] // Short flag (-f) and long flag (--file)
    file: String,
}

fn main() {
    let args = Args::parse(); // Parse command-line arguments
    match count_words(&args.file) {
        Ok(count) => println!("Words count: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
    match count_chars(&args.file) {
        Ok(count) => println!("Chars count: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
    match count_lines(&args.file) {
        Ok(count) => println!("Lines count: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
    match word_frequency(&args.file) {
        Ok(freq) => {
            println!("word frequency:");
            for (word, count) in freq {
                println!("{}: {},", word, count);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    println!("\n\n");
    let a: i32 = 10;
    let b: i32 = 6;
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("a * b = {}", a * b);
    println!("a / b = {}", a / b);
    if a > b {
        println!("a > b")
    } else {
        println!("b > a")
    }
    let numbers: [i32; 4] = [2, 4, 6, 8];
    for num in numbers {
        // println!(" show num: {}", num);
    }
    let mut start = 1;
    let end = 5;
    while start <= end {
        // println!("while show: {}", start);
        start += 1;
    }
    start = 1;
    loop {
        if start > end {
            break;
        }
        // println!("loop show: {}", start);
        start += 1;
    }
    let sum = add(10, 20);
    println!("Sum: {}", sum);

    let greeting = say_hello("World");
    println!("{}", greeting);
}
