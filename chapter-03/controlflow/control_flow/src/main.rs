fn main() {
    let number = 6;
    if number < 5 {
        println!("This condition is True");
    } else {
        println!("This condition is false");
    }
    //if number{}-->if block only accepts boolean condition
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Numebr is divisible by 2");
    } else {
        println!("number is not divisible by 4,3,2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The number is {}", number);
    loops();
}
fn loops() {
    let mut counter = 0;
    println!("------------------------------");
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result value is {}", result);
    loops_inside_loops();
}
fn loops_inside_loops() {
    let mut count = 0;
    println!("--------------------------------");
    'counting_up: loop {
        println!("count is:{}", count);
        let mut remaining = 10;
        loop {
            println!("Remaining is:{}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count is {}", count);
    loop_while();
}
fn loop_while() {
    println!("-------------------");
    let mut count = 3;
    while count != 0 {
        println!("count is {}", count);
        count -= 1;
    }
    println!("END OD LOOP");
    loop_through_collections();
}
fn loop_through_collections() {
    println!("-----------------");
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index < 5 {
        println!("a[{}] is {}", index, a[index]);
        index += 1;
    }
    println!("---------------------");
    for number in a {
        print!("element is {} ", number);
    }
    println!();
}
