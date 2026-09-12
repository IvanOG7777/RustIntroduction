fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for val in list {
        if val > largest {
            largest = val;
        }
    }

    largest
}

// sort of like c++ templates
// function is any type
// Will return passed types largest
// without PartialOrd, compiles complains because we can pass non-primitive types that cant to the > operator
// std::cmp::PartialOrd guarantees the compiler that the passed type will be able to be compared using <,>,=,>=,<=
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for val in list {
        if val > largest {
            largest = val;
        }
    }

    largest
}
fn main() {

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("Largest in the number list is: {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("Largest in the number list is: {result}");
    println!();

    // using template
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("Largest in the number list is: {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("Largest in char list is: {result}");
    }
}