pub mod data_types;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // create constant u32 bit immutable value
// valid within the entire programs life.
fn main() {
    let x = 5; // immutable
    println!("Value of x is: {x}");
    // x = 6; // invalid at this point x has been used and deleted
    println!("New value of x is: {x}");

    let mut y = 5;
    println!("Value of y is: {y}");
    y = 6;
    println!("Value of y is: {y}");

    let z = 5;

    let z = z + 1; // shadows first instances of z

    // create another scope within main
    {
        let z = z * 2; // this instance of z shadows z = 6 above, does 6 * 2 = 12;
        println!("Value of z is: {z}");
    }

    // Once inner shadowing dies, z is returned back to 6;
    println!("The value of z is: {z}");

    let spaces = "   ";
    let spaces = spaces.len();

    println!("Length of spaces is: {spaces}");
}