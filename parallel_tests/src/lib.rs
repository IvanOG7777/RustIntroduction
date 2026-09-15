fn prints_and_returns(a: i32) -> i32{
    println!("I got the value: {a}");
    10 // always returns 10 at the end of function call
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns(4);
        assert_eq!(value, 10);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns(8);
        assert_eq!(value, 5);
    }
}
