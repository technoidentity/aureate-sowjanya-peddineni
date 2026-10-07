fn main() {
    println!("Singular Data Types in Rust!");
    let x=2.0;//By default f64 type
    let y:f32=3.2;//f32
    println!("x is {x} and y is {y}");
    let sum=5+10;
    let differnece =54.6-34.3;
    let truncated=-5/3;//divide and give integer value of quotient
    println!("sum is {sum}, difference is {differnece} and truncated value is {truncated}");
    let t=true;
    let f:bool=false;
    println!("t is {t} and f is {f}");
    let c='z';
    let z:char='ℤ';
    let heart_ey='❤';
    println!("c is {c} and z is {z}");
    println!("heart_ey is {heart_ey}");
    println!("Compound Data Types in Rust!");
    println!("Tuples are grouping together different types of values and have fixed size,they can not grow/shrink");
    let tup:(i32,f64,u8)=(500,6.4,1);
    println!("tup is {tup:?}");
    let (a,b,c)=tup;//This is called destructuring of a tuple
    println!("a is {a}, b is {b}, and c is {c}");
    println!("Accessing tuple values using index");
    println!("The first value of tup is {} and the second value is {} and the third value is {}",tup.0,tup.1,tup.2);
    println!("Arrays are grouping together same types of values and have fixed size,they can not grow");
    let arr=[1,2,3,4,5];
    println!("arr is {arr:?}");
    let a:[i32;5]=[1,2,3,4,5];//5 elements of type i32
    println!("a is {a:?}");
    println!("Accessing array values using index");
    println!("The first value of a is {} and the second value is {}",a[0],a[1]);



}
