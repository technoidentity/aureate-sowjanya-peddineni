fn main() {
    println!("Variables in Rust!");
    let x=5;
    println!("The value of x is:{x}");
    //x=6;
    //println!("The value of x is :{x}");
    let x=x+1;
    {
        let x=x*2;
        println!("The value of x in the inner sapce is :{x}");

    }
    println!("The value of x in outer space is :{x}");

}
