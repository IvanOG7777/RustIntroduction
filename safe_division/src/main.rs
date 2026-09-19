use std::num::ParseFloatError;
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Result::Err(String::from("Cannot divide by 0")); // return Err as string if b is 0
    }

    Result::Ok(a / b) // return ok if both values are good
}

fn calculate_match(a: &str, b: &str) -> Result<f64, String> {
    let a_as_num: f64 = match a.trim().parse() {
        Ok(num) => num, // return ok a float 64 number
        Err(_) => return Err(String::from("A is not a number")), // or if of type return a string
    };

    let b_as_num: f64 = match b.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(String::from("B is not a number")),
    };

    divide(a_as_num, b_as_num)
}

fn calculate_operator(a: &str, b: &str) -> Result<f64, String> {
    let a_as_num: f64 = a.trim().parse().map_err(|err: ParseFloatError| err.to_string())?; // turn PareFloatError to string. ? either return valid number or err as String object
    let b_as_num: f64 = b.trim().parse().map_err(|err: ParseFloatError| err.to_string())?;

    divide(a_as_num, b_as_num)
}

fn main() {
    let result = calculate_match("2", "bb");
    let result_1 = calculate_operator("2", "s");

    println!("Total is: {result:?}");
    println!("Total is: {result_1:?}");
}
