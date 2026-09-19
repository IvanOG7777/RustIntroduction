use std::error::Error;
use std::{env, fmt, fs};
use std::fmt::{Formatter, Display};

struct FileStats {
    characters: usize,
    words: usize,
    lines: usize,
}
impl FileStats {

    fn new_stats() -> FileStats {
        FileStats {
            characters: 0,
            words: 0,
            lines: 0,
        }
    }
    fn print_stats(&self) {
        println!("Total characters: {}", self.characters);
        println!("Total words: {}", self.words);
        println!("Total lines: {}", self.lines);
    }
}
#[derive(Debug)]
enum Errors {
    ReadToStringError(),
    FileEmptyError(),
}

// Extend Display to Errors enum
impl Display for Errors {
    // tell formater how to use enum errors
    // returning new error types messages
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // match the error enum type
        match self {
            Errors::FileEmptyError() => {
                // push custom string to formater buffer
                write!(f, "File doesnt contain any material")
            },

            Errors::ReadToStringError() => {
                // push custom string to formater buffer
                write!(f, "Failed to read file to string")
            }
        }
    }
}

// Only really need the declaration no need to add anything
impl Error for Errors {}
fn analyze_file(path: &str) -> Result<FileStats, Box<dyn Error>> {
    let contents = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return Err(Box::new(Errors::ReadToStringError())),
    };

    if contents.len() <= 0 {
       return Err(Box::new(Errors::FileEmptyError()))
    }

    let mut stats = FileStats::new_stats();

    for _word in contents.split_whitespace() {
        stats.words += 1;
    }

    for _line in contents.lines() {
        stats.lines += 1;
    }

    for _char in contents.chars() {
        stats.characters += 1;
    }

    Ok(stats)
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let file = &args[1];

    let result = analyze_file(file);

    match result {
        Ok(stats) => {
            stats.print_stats()
        }

        Err(e) => println!("Error: {e}")
    }
}
