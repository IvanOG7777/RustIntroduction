use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // get a copy of team_name score if it exits in the hashmap or return 0

    println!("{score}");

    for (key, value) in &scores {
        println!("Key: {key}, value: {value}");
    }

    {
        let field_name = String::from("Favorite Color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // invalid at this point since map takes ownership of the data
        // println!("{field_name}");
        // println!("{field_value}");

        for (key, value) in &map {
            println!("Key: {key}, value: {value}");
        }
    }

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25); // overwrites 10 with 25

        println!("{scores:?}");
    }
}