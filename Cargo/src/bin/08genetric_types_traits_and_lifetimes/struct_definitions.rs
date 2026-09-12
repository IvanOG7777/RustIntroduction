// allows variable types to be passed to struct
struct Point<T> {
    x: T,
    y: T,
}

// allows for two different types to be passed
struct PointNew<T, U> {
    x: T,
    y: U,
}
fn main() {
    let integer_point = Point {x: 1, y: 2}; // creates an integer type of point
    let float_point = Point {x: 1.0 ,y: 2.0}; // creates a float type of point

    // let wont_work = Point {x: 1, y: 2.0}; // Won't work because we are passing two different types to the struct, expects the struct to be of first declared type


    let integer_float_point = PointNew {x: 1, y: 2.0};
    let float_integer_point = PointNew {x: 2.0, y: 1};
}