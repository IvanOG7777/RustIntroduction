
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess must be greater than or equal to 1, got {value}.");
        } else if value > 100 {
            panic!("Guess must be greater than or equal to 100, got {value}.");
        }
        Guess {value}
    }
}

pub fn add_two(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(1000);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add_two(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}
