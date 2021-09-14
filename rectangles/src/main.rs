#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let r = Rectangle {
        width: 30,
        height: 50,
    };

    let o = Rectangle {
        width: 20,
        height: 20,
    };
    
    println!("Can hold: {:?}, Area: {:?}", r.can_hold(&o), r.area());
}

