pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 3
}

pub fn greeting(name: &str) -> String {
    // format!("Hello: {name}") // returns String object with name str literal attached to it, passes test

    String::from("Hello") // fails test since doesn't contain "Carol" within
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn it_adds_two() {
    //     let result = add_two(2);
    //     // fails since left doesn't equal right at execution
    //     assert_eq!(result, 4); // check if left and right are equal
    //     // assert_ne!(result, 4); // check if left and right are NOT equal
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 7,
            height: 6,
        };

        assert!(larger.can_hold(&smaller)); // assert if a bool condition is true
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 7,
            height: 6,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
            "Greetings didn't contain name, value was: `{result}"
        );
    }
}
