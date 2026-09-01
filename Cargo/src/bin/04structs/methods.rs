struct Rectangle {
    width: u32,
    height: u32,
}

// use impl (implementation) keyword to define a function within the context of Rectangle object
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// You can write methods and function names of an object with the same name as declared variables within
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}
impl Rectangle {
    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}
// You can have these methods separated or all in one impl
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//     fn width(&self) -> bool {
//         self.width > 0
//     }
//
//     fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
//         self.width > other_rectangle.width &&  self.height > other_rectangle.height
//     }
// }

//Not a method since we don't need an instance of a Rectangle Object
// This implementation works like a regular function
// Will return a Rectangle object with width and height of size, size
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {}, square pixels", rect1.area());

    if rect1.width() {
        println!("The rectangle ha a nonzero width; it is: {}", rect1.width);
    }

    {
        let rect1 = Rectangle{
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle{
            width: 10,
            height:40,
        };

        let rect3 = Rectangle{
            width: 60,
            height: 45
        };

        println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3 {}", rect1.can_hold(&rect3));
    }

    let rect2 = Rectangle::square(50);
    println!("The width and height of rect2 are {} and {}", rect2.width, rect2.height);
}