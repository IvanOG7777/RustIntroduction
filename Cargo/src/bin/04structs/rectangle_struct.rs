#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels", area(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels", tuple_area(rect1)); // pass whole tuple to function

    // Declare a rectangle object
    let rect2 = Rectangle {width: 30,height: 50,};

    println!("The area of the rectangle is {} square pixels", rect_object_area(&rect2)); // pass whole tuple to function

    println!("rect2 is: {rect2:?}"); // print all elements of current Rectangle object

    dbg!(&rect2); // print line of declaration of rect2, its type and what been initialized within
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//Pass in a tuple variable to function
// returns unsigned 32-bit integer
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1 // access variables with pass name at index 0 and 1
}

// function that takes a reference of a rectangle object
// returns unsigned 32-bit integer
fn rect_object_area(passed_rectangle: & Rectangle) -> u32 {
    passed_rectangle.width * passed_rectangle.height
}