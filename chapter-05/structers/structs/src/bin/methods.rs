#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width &&self.length>other.length
    }
    fn square(size:u32)->Self{
        Self{
            width:size,
            length:size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 20,
        length: 30,
    };
    println!("The area is {}", rect1.area());
    let rect2=Rectangle{
        width:10,
        length:20,
    };
    let rect3=Rectangle{
        width:30,
        length:40,
    };
    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",rect1.can_hold(&rect3));
    let sq=Rectangle::square(10);
    println!("The square is {:?}",sq);
}
