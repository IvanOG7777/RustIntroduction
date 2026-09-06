fn find_number(int_array: [i32; 5], number_to_find: i32) -> Option<usize> {
    for i in 0..5 {
        if int_array[i] == number_to_find {
            return Some (i);
        }
    }
    None
}
// Option<t> is cool
fn main() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let result = find_number(array, 5);

    match result {
        Some(x) => println!("Found number at index: {x}"),
        None => println!("Failed to find the number")
    }
}