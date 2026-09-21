pub fn validate_age(age: i32) -> Result<i32, String> {
    if age < 0 {
        Err(String::from("Age cannot be less than 0"))
    } else if age > 130 {
        Err(String::from("You are old asf"))
    } else {
        Ok(age)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_age_returns_ok() {
        let result = validate_age(22);

        assert!(result == Ok(22));
    }

    #[test]
    fn old_age_returns_err() {
        let result = validate_age(300);

        assert_eq!(result, Ok(22));
    }

    #[test]
    fn negative_age_returns_err() {
        let result = validate_age(-1);

        assert_eq!(result, Ok(22));
    }
}
