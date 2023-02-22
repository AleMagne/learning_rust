#[derive(Debug)]
struct Rectangle{
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.height * self.width
    }
}
fn main() {
    let mut polygon1 = Rectangle {
        width: 32,
        height: 40,
    };

    println!("the rectangle is {:?}", polygon1);

    polygon1.width = 40;

    println!("the rectangle is {:?}", polygon1);

    let area = polygon1.area();
    println!("The area is: {area}");
}
