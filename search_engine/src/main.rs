use std::{env, fs, process};
use std::error::Error;
use std::sync::mpsc::SyncSender;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
    count: bool
}

enum ReturnType<'a> {
    Vector(Result<Vec<&'a str>, &'static str>),
    Count(u32),
    Err(&'a str)
}

impl Config {

    fn new_config(passed_query: &String, passed_file_path: &String, ignore_case: bool, count: bool) -> Config {

        let query = passed_query.clone();
        let file_path = passed_file_path.clone();
        Config {
            query,
            file_path,
            ignore_case,
            count
        }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments")
        }

        let query = &args[1];
        let file_path = &args[2];
        let feature = &args[3];

        let mut ignore_case = false;
        let mut count = false;

        if feature == "--ignore_case" {
            ignore_case = true;
        } else if feature == "--count" {
            count = true
        } else {
            return Err("Invalid feature argument");
        }


        let config = Config::new_config(query, file_path, ignore_case, count);

        Ok(config)
    }
}

fn search<'a> (query: &str, contents: &'a str) -> ReturnType<'a> {
    if contents.len() <= 0 {
        return ReturnType::Err("No content was provided to search for")
    }

    let mut return_vector: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            return_vector.push(line);
        }
    }

    ReturnType::Vector(Ok(return_vector))
}

fn search_ignore_case<'a> (query: &str, contents: &'a str) -> ReturnType<'a> {
    if contents.len() <= 0 {
        return ReturnType::Err("No content was provided to search for")
    }

    let mut return_vector: Vec<&'a str> = Vec::new();
    let query_to_lower = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_to_lower) {
            return_vector.push(line);
        }
    }

    ReturnType::Vector(Ok(return_vector))
}

fn search_count<'a> (query: &str, contents: &'a str) -> ReturnType<'a> {
    if contents.len() <= 0 {
        return ReturnType::Err("No content was provided to search for")
    }

    let mut count: u32 = 0;

    for line in contents.lines() {
        if line.contains(query) {
            count += 1;
        }
    }


    ReturnType::Count(count)
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let read_to_string_result = fs::read_to_string(config.file_path);

    let contents = match read_to_string_result {
        Ok(built_contents) => built_contents,

        Err(e) => {
            return Err(Box::new(e));
        }
    };

    let mut search_result = ReturnType::Err("No matching cases in run");
    if config.ignore_case == false && config.count == false {
        search_result = search(&config.query, &contents);
    } else if config.ignore_case == true {
        search_result = search_ignore_case(&config.query, &contents);
    } else if config.count == true {
        search_result = search_count(&config.query, &contents);
    }

    match search_result {
        ReturnType::Vector(built_vector) => {

            match built_vector {
                Ok(valid_lines) => {
                    for line in valid_lines {
                        println!("{line}");
                    }
                }
                Err(e) => println!("{}", e),
            }
        }

        ReturnType::Count(count) => {
            println!("There are {} references to the query: {}", count, config.query);
        }

        ReturnType::Err(e) => {
            print!("Something went wrong, error is: {e}");
        }
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
