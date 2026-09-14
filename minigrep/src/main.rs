use std::{env, process};
use std::fs;
// use std::error;
use std::error::Error;
use minigrep::{search}; // import functions from lib

fn main() {
    // works like argc, argv in c/c++
    // index 0 is executable, after that are the actual arguments starting from 1
    let args: Vec<String> = env::args().collect();

    // let query = &args[1]; //  item were searching for
    // let file_path= &args[2]; // path we want to look for it in

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem with parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file {}", config.file_path);

    // read all contents from file_path and turn to String object
    // if failed to read panic! to error msg
    let contents = fs::read_to_string(&config.file_path).expect("Failed to read from file");

    println!("With text:\n {contents}");

    println!();
    println!();
    println!("Using run function");
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//
//     Config { query, file_path }
// }

impl Config {
    // fn new(args: &[String]) -> Config {
    //
    //     if args.len() < 3 {
    //         println!("Not enough arguments")
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //
    //     Config { query, file_path }
    // }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})

    }
}

// return nothing or dynamic error
fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // if success give success value else exception

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}