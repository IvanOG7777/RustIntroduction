pub mod utf8_strings;

fn main() {
    // creates variable v as a Vector of i32 values
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3]; // inferred vector of i32 values

    let mut v3: Vec<i32> = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5]; // create inferred i32 vector
    // before handing reference of v4[2] to third
    for val in &v4 {
        println!("{val}");
    }
    println!();


    let third: &i32 = &v4[2]; // reference a value within the vector at index 2
    println!("The third element is {third}");

    // after handing reference of v4[2] to third
    for val in &v4 {
        println!("{val}");
    }
    println!();

    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("No third element"),
    }

    // after .get method
    for val in &v4 {
        println!("{val}");
    }
    println!();

    // causes panic out of bounds error
    // let v5 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v5[100];
    // let does_not_exist = v5.get(100);

    let mut v6 = vec![1, 2, 3, 4, 5]; // owner
    let first = &v6[0]; // takes a borrow of element at index 0

    // v6.push(6); // cant do this since an immutable borrow exists

    println!("The first element is: {first}");

    for val in &v6 {
        println!("{val}");
    }

    let mut v7 = vec![100, 32, 57];
    for i in &mut v7 {
        *i += 50;
    }

    println!("After adding");
    for i in &v7 {
        println!("{i}");
    }

    {
        enum SpreadsheetCell {
            Int(i32),
            Float(f32),
            Text(String),
        }

        // creates vector of type SpreadsheetCell, which can hold different types
        let row  = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Text(String::from("Blue")), SpreadsheetCell::Float(10.12)];

        for val in &row {
            match val {
                SpreadsheetCell::Int(value) => println!("{value}"),
                SpreadsheetCell::Float(value) => println!("{value}"),
                SpreadsheetCell::Text(value) => println!("{value}"),
            }
        }
    }
}
