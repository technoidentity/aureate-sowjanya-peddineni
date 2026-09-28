fn main() {
    let width = 34;
    let length = 20;
    let rect = (20, 30);
    println!("The area of rectangel is {}", area(width, length));
    println!("The area is {}", rectarea(rect));
    let rect1 = Rectangle {
        width: 40,
        length: 60,
    };
    println!("{:?}", rect1);
    println!("The Rectangle area is {}", rarea(&rect1));
    println!(
        "printing the instance of rectangle by using debug {:#?}",
        rect1
    );
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        length: 12,
    };
    dbg!(&rect2);
}
fn area(width: i32, length: i32) -> i32 {
    width * length
}
fn rectarea(dim: (i32, i32)) -> i32 {
    dim.0 * dim.1
}
#[derive(Debug)]
struct Rectangle {
    width: i32,
    length: i32,
}
fn rarea(rectangle: &Rectangle) -> i32 {
    rectangle.width * rectangle.length
}
