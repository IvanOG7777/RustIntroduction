fn smallest<T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut smallest_val = &list[0];

    for val in list {
        if val < smallest_val {
            smallest_val = val;
        }
    }

    smallest_val
}

fn min_max<T: std::cmp::PartialOrd>(list: &[T]) -> Option<(&T, &T)> {

    if list.len() == 0 {
        return None
    }

    let mut largest_val = &list[0];

    for val in list {
        if val > largest_val {
            largest_val = val;
        }
    }

    let smallest_val = smallest(list);

    Some((smallest_val, largest_val))
}

fn main() {

    let numbers: Vec<i32> =  vec![8, 2, 19, 4, 10];

    let result = min_max(&numbers);

    match result {
        Some(tuple) => println!("Min and Max of array is: ({}, {})", tuple.0, tuple.1),

        None => println!("Numbers vector is empty"),
    }

}
