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

// implementation on all passed T types
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// implementation on only types with f32 value
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// describes the types this implementation will use,
// Generic self's X1, Y1 values
impl<X1, Y1> Point2<X1, Y1> {
    // describes the types this method call will use
    // X2, Y2
    // have other be of these generic types
                                                    // Return a new point with self generic X1 and other generic Y2
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}
fn main() {
    let integer_point = Point { x: 1, y: 2 }; // creates an integer type of point
    let float_point = Point { x: 1.0, y: 2.0 }; // creates a float type of point

    // let wont_work = Point {x: 1, y: 2.0}; // Won't work because we are passing two different types to the struct, expects the struct to be of first declared type

    let integer_float_point = PointNew { x: 1, y: 2.0 };
    let float_integer_point = PointNew { x: 2.0, y: 1 };

    let p = Point { x: 5, y: 10 };

    println!("p.x is: {}", p.x());

    let f_p = Point { x: 3.2, y: 2.2 };
    println!(
        "The distance from origin is: {}",
        f_p.distance_from_origin()
    );

    let i_p = Point { x: 2, y: 3 };
    // println!("The distance from origin is: {}", i_p.distance_from_origin()); // error since type is of integer and implementation is only for f32 values

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let new_point = p1.mixup(p2);

    println!("New point is : {}, {}", new_point.x, new_point.y);
}
