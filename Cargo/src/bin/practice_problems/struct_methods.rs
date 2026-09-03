struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width * self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_fit(&self, passed_rectangle: &Rectangle) -> bool {
        self.width > passed_rectangle.width && self.height > passed_rectangle.height
    }
}
fn main() {
    let rectangle1 = Rectangle {width: 10, height: 24};
    let rectangle2 = Rectangle {width: 9, height: 10};
    let rectangle3 = Rectangle {width: 9, height: 25};

    println!("Area: {}", rectangle1.area());
    println!("Perimeter: {}", rectangle1.perimeter());
    println!("Is square: {}", rectangle1.is_square());
    println!("Can rectangle2 fit in rectangle 1: {}", rectangle1.can_fit(&rectangle2));
    println!("Can rectangle3 fit in rectangle 1: {}", rectangle1.can_fit(&rectangle3));
}