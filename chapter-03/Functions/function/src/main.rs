fn main() {
    println!("Hello,world!");
    another_function();
    another_function_withargument(5);
    another_function_with_multipleargument(10, 'm');
    let x = function_with_return(9);
    println!("the value is {}", x);
}
fn another_function() {
    println!("Another function ");
}
fn another_function_withargument(x: i32) {
    println!("X value is {}", x);
}
fn another_function_with_multipleargument(x: i32, d: char) {
    println!("The distance is {} {}", x, d);
}
fn function_with_return(x: i32) -> i32 {
    x + 1
}
