fn main() {
    let user1 = User {
        username: String::from("sowjanya"),
        email: String::from("sowjanyapeddineni@gmail.com"),
        active: true,
        sign_in_count: 4,
    };
    let user2 = User {
        username: String::from("Priyanka"),
        ..user1
    };
    println!("Users name is {}", user1.username);
    //println!("User email is {}", user1.email);
    println!("User active status{}", user1.active);
    println!("User sign in count {}", user1.sign_in_count);
    let x = build_user(String::from("Thriveni"), String::from("Thriveni@gmail.com"));
    println!("{}", x.username);
    println!("{}", x.email);
    println!("Users name is {}", user2.username);
    println!("User email is {}", user2.email);
    println!("User active status{}", user2.active);
    println!("User sign in count {}", user2.sign_in_count);
    let _black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Point(x, y, z) = origin;
    println!("{x} {y} {z}");
    let mut p = Coord { x: 0, y: 0 };
    let z = &mut p.x;
    *z = *z + 1;
    println!("{} {}", x, p.x);
}
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Coord {
    x: i32,
    y: i32,
}
