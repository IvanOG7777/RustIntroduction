use std::io;
fn main() {
    // let guess = "42".trim().parse().expect("Not a number"); needs to be cast into proper integer type u32, i32, usize, etc

    let guess: usize = "42".trim().parse().expect("Not a number"); // Proper cast

    println!("Guess is: {guess}");

    let x = 2.0; // inferred 64-bit floating number

    let y: f32 = 3.0; // statically cast 32-bit floating number


    // int are implicitly cast as i32 while floating values are f64
    let sum = 5 + 10;

    let different = 99.5 - 44.3;

    let product = 3 * 30;

    let quotient = 56.7 / 32.3;

    let truncated = -5 / 3;

    let remainder = 43 % 5;

    let t = true; // implicit

    let f: bool = false; // explicit

    let c = 'z';
    let z: char = 'z';
    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");

    // Compounded types
    // Explicitly declare variable types for tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); // considered as one value
    let (x, y, z) = tup;

    println!("Value of y is: {y}");

    let i32 = tup.0;
    let f64 = tup.1;
    let u8 = tup.2;

    println!("Value of x is: {i32}");
    println!("Value of x is: {f64}");
    println!("Value of x is: {u8}");

    //Array types
    let a = [1, 2, 3, 4, 5];
    // implicitly declares array of 12 string values
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // explicitly declares array as 5 i32 bit values
    let a32: [i32; 5] = [6, 7, 8, 9, 10];

    //creates array of 5 i32 bit 3's
    let a3 = [3; 5];

    println!("Implicit array");
    for val in a {
        print!("{val} ");
    }
    println!();

    println!("Months array");
    for month in months {
        print!("{month} ");
    }
    println!();

    println!("Explicit i32 array");
    for val in a32 {
        print!("{val} ");
    }
    println!();

    println!("Explicit value and count array");
    for val in a3 {
        print!("{val} ");
    }
    println!();

    let first = a[0];
    let second = a[1];

    println!("First and second element of array a");
    println!("{first}");
    println!("{second}");

    let b = [1, 2, 3, 4, 5];

    println!("Enter an array index!");

    let mut index = String::new();

    io::stdin().read_line(& mut index).expect("Failed to read line");

    // Only continues to 0 if entered string cannot be made into a number
    let mut index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    if (index > 4) {
        index = 0;
    }

    let element = b[index];

    println!("Element at: {index} is: {element}");
}