fn main() {
    let word = String::from("");

    let first = first_word(word);

    println!("The first word in the String is: {first}");
}

fn first_word(passed_string: String) -> String {
    let string_as_bytes = passed_string.as_bytes();
    let mut index_count = 0;

    for i in 0..string_as_bytes.len() {
        if string_as_bytes[i] == b' ' {
            break;
        }
        index_count += 1;
    }

    let first = String::from(&passed_string[0..index_count]);

    if index_count == passed_string.len() {
        return passed_string;
    }

    first
}