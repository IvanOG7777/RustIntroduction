
// Tuple declared i32 structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual; // unit-like struct
// dont really know why or how this is useful yet
fn main() {
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let Point(x, y, z) = origin;

    println!("{x}, {y}, {z}");

    let subject = AlwaysEqual;
}