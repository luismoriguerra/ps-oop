
mod geometry {
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
    }

    impl Rectangle {

        pub fn new(width: u32, height: u32)->Rectangle {
            Rectangle { width, height }
        }

        pub fn area(&self)->u32 {
            self.width * self.height
        }
    }
}


fn main() {
    let rec = geometry::Rectangle { width: 30, height: 50 };
    let rec2 = geometry::Rectangle::new(30, 50);

    println!("The area of the rectangle is {} square pixels.", rec.area());
    println!("The area of the rectangle is {} square pixels.", rec2.area());
}
