use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn most_common_word<'a>(map: &mut HashMap<&'a str, i32>) -> Option<(& 'a str, i32)> {


    if map.len() == 0 {
        return None;
    }

    let (first_word, first_count) = map.iter_mut().next().unwrap();
    let mut word = first_word.clone();
    let mut count = first_count.clone();

    for (key, value) in map.into_iter() {
        if *value > count {
            count = *value;
            word = key;
        }
    }

    Some((word, count))
}

fn main() {

    let mut word_map: HashMap<&str, i32> = HashMap::new(); // create new map

    let text = "Mutability and immutability borrows and references in Rust can be so confusing for me. I wonder when I will be able to learn this stuff properly";
    let text = "rust is fast and rust is safe and rust is fun";
    let text = "I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

    // loop though text extracting each word as a token
    for token in text.split_whitespace() {
        match word_map.entry(token) {
            Occupied(entry) => { // if slot is occupied, just add one to count
                let word_count = entry.into_mut();
                *word_count += 1;
            },

            Vacant(new_entry) => { // if slot is vacant, just insert 1 to count
               new_entry.insert(1);
            }
        }
    }

    for (word, count) in &word_map {
        println!("Word: {}, count: {}", word, count);
    }

    let result = most_common_word(&mut word_map);

    match result {
        Some(common_value) => {
            println!("Most common word is: {}, with a frequency of: {}", common_value.0, common_value.1);
        },

        None => {
            println!("Couldn't find word or map is empty ");
        }
    };
}
