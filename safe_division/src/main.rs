fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Result::Err(String::from("Cannot divide by 0"));
    }

    Result::Ok(a / b)
}

fn calculate(a: &str, b: &str) -> Result<f64, String> {
    let a_as_num: f64 = a.parse()?;
    let b_as_num: f64 = b.trim().parse().expect("B is not a number");

    let result = divide(a_as_num, b_as_num);

    result
}

fn main() {
    let result = calculate("10", "2");
    match result {
        Ok(num) => println!("Total is: {num}"),

        Err(err) => println!("{err}")
    }
}
