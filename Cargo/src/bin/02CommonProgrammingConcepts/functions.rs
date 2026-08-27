fn main() {
    println!("Hello from main scope");

    another_function();
    parameter_function(10);
    print_labeled_measurement(5, 's');

    let mut x = {
        let y = 3;
        y + 1
    };

    println!("Value of x is: {x}");

    x = five();
    println!("Value of x is: {x}");

    x = plus_one(x);
    println!("Value of x is: {x}");
}

fn another_function() {
    println!("Hello from another_function");
}

fn parameter_function(x: i32) {
    println!("Value passes to parameter_function() is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}