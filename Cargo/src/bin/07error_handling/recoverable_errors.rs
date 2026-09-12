use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // let greeting_file_result = File::open("hello.txt"); // try to open file with open function and file name
    // // returns a result type either Ok or Err
    //
    // // get actual file with match
    // let greeting_file = match greeting_file_result {
    //     Ok(File) => File, // if return type is Ok return file
    //     Err(error) => panic!("Problem with opening the file: {error:?}"), // else panic out
    // };


    {
        let file_name = "hello.txt";
        let greeting_file_result = File::open(file_name); // try to open file with open function and file name
        // returns a result type either Ok or Err

        // get actual file with match
        let _greeting_file = match greeting_file_result {
            Ok(file) => file, // if return type is Ok return file
            Err(error) => match error.kind() { // if failed ask what type of faillure it was
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(new_file) => new_file, // if error is not found, create new file and return it
                    Err(e) => panic!("Problem with creating new file: {e:?}"), // anything else panic
                },
                _ => panic!("Problem with openning file: {error:?}"),
            }
        };

        println!("File: {file_name}, has been opened/created");
    }

    {
        let file_name = "hello.txt";

        let greeting_file_result = File::open(file_name).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(file_name).unwrap_or_else(|error| {
                    panic!("Problem with creating the file: {error:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });

        println!("File: {file_name} has been opened/created");


    }

    {
        let file_name = "helloworld.txt";
        let _greeting_file = File::open(file_name).unwrap(); // auto panics for us
    }

    {
        let file_name = "helloWold.txt";
        let greeting_file = File::open(file_name).expect("helloWorld.txt should have been included in this project");
    }
}