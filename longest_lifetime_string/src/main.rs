// pass in two
fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_three<'a> (x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    if x.len() > y.len() && x.len() > z.len() {
        x
    } else if y.len() > x.len() && y.len() > z.len() {
        y
    } else {
        z
    }
}


fn main() {

    let mut result: &str;
    {
        let string2 = String:: from("Rust");
        let string1 = String::from("Hello world");
        let longest = longest(&string1, &string2);

        result = longest;

        println!("This value survived the scope above: {}", result); // valid
    }
    // println!("This value survived the scope above: {}", result); // invalid since we are trying to access string1 once it's been dropped

    {
        let one = String::from("Hello");
        let two = String::from("Hello world");
        let three = String::from("Rust");

        let longest = longest_three(&one, &two, &three);

        result = longest;
        println!("Longest is: {result}");
    }

    // println!("This value survived the scope above: {}", result); //Same here, invalid since we are trying to access two once it's been dropped
}
