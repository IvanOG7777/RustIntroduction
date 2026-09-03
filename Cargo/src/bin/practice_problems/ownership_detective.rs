//TODO: solve ownership issues
fn main() {
    // let s1 = String::from("Rust");
    // let s2 = s1; // ownership is passed to s2, s1 goes out of scopes
    //
    // println!("{s1}"); // error cant print this since s1 isn't an owner
    // println!("{s2}");


    //One of the solutions
    let s1 = String::from("Rust");
    let s2 = &s1; // give s2 a reference to s1

    // Allows for both s1 and s2 to print
    println!("{s1}");
    println!("{s2}");

    // Another solution
    let s1 = String::from("Rust");
    let s2 = s1.clone(); // clone object to s2

    println!("{s1}");
    println!("{s2}");
}