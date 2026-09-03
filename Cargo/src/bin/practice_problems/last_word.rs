fn main() {
    let word = String::from("hello world from rust");

    let last = last_word(word);

    println!("The last word in the string is:{last}");

}

fn last_word(passed_string: String) -> String {
    let string_as_bytes = passed_string.as_bytes();
    let mut space_index = 0;

    for i in 0..string_as_bytes.len() {
        if string_as_bytes[i] == b' ' {
            space_index = i;
        }
    }

    let last = String::from(&passed_string[space_index..]);

    last
}