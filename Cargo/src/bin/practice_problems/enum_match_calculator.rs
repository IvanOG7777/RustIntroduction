use crate::Operation::{Add, Subtract, Multiply, Divide};
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(a: i32, b: i32, operation: Operation) {
    match operation {
        Add => {
            let sum = a + b;
            println!("{} + {} = {}", a, b, sum);
        }

        Subtract => {
            let sum = a - b;
            println!("{} - {} = {}", a, b, sum);
        }

        Multiply => {
            let sum = a * b;
            println!("{} * {} = {}", a, b, sum);
        }

        Divide => {
            if b == 0 {
                println!("Cant divide by 0");
            }

            let sum = a / b;
            println!("{} / {} = {}", a, b, sum);
        }
    }
}

fn main() {

    calculate(10, 2, Add);
    calculate(10, 2, Subtract);
    calculate(10, 2, Multiply);
    calculate(10, 2, Divide);

    calculate(0, 2, Divide);

    calculate(10, 0, Divide);
}