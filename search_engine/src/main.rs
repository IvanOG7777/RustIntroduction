use std::{env, fs, process};
use std::error::Error;

struct Config {
    query: String,
    file_path: String,
}

impl Config {

    fn new_config(passed_query: &String, passed_file_path: &String) -> Config {

        let query = passed_query.clone();
        let file_path = passed_file_path.clone();
        Config {
            query,
            file_path
        }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments")
        }

        let query = &args[1];
        let file_path = &args[2];

        let config = Config::new_config(query, file_path);

        Ok(config)
    }
}

fn search<'a> (query: &str, contents: &'a str) -> Result<Vec<&'a str>, &'static str> {
    if contents.len() <= 0 {
        return Err("No content was provided to search for")
    }

    let mut return_vector: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
            if line.contains(query) {
                return_vector.push(line);
            }
    }

    Ok(return_vector)
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let read_to_string_result = fs::read_to_string(config.file_path);

    let contents = match read_to_string_result {
        Ok(built_contents) => built_contents,

        Err(e) => {
            return Err(Box::new(e));
        }
    };

    let search_result = search(&config.query, &contents);

    let valid_lines = match search_result {
        Ok(line_vector) => line_vector,

        Err(e) => {
            return Err(e.into())
        }
    };

    for line in valid_lines {
        println!("{line}");
    }

    Ok(())
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let build_result = Config::build(&args);

    let config = match build_result {
        Ok(built_config) => built_config,

        Err(e) => {
            println!("{e}");
            process::exit(1);
        },
    };

    let result = run(config);

    println!();
    match result {
        Ok(_) => println!("OK!"),

        Err(e) => println!("Something failed: {e}")
    }
}
