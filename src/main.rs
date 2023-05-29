use std::io;
use std::process;

fn main() {
    // let mut name = "seb";
    // // can add let mut name = to make it mutable and change the value later
    //
    // println!("{}", name);
    //
    // name = "Maz";
    //
    // println!("{}", name);
    //
    // let car = "audi";
    // let truck = "ford".to_string();
    //
    // println!("{} and {}", car, truck);
    //
    // say_name(&car, &truck);
    // say_name(&car, &truck);

    // println!("Please Enter your Name: ");
    //
    // let mut entered_name = String::new();
    //
    // io::stdin().read_line(&mut entered_name);
    // println!("Hello {}", entered_name);

    println!("type first number : ");

    let mut first = String::new();
    io::stdin().read_line(&mut first);
    let a: u32;
    match first.trim().parse() {
        Ok(val) => {
            a = val;
        }
        Err(_err) => {
            println!("This is not a Valid number!");
            process::exit(1);
        }
    }

    println!("type second number : ");

    let mut second = String::new();
    io::stdin().read_line(&mut second);
    let b: u32;
    match second.trim().parse() {
        Ok(val) => {
            b = val;
        }
        Err(_err) => {
            println!("This is not a Valid number!");
            process::exit(1);
        }
    }

    let result = sum(a, b);

    println!("{} + {} = {}", a, b, result)
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

// fn say_name(first: &str, last: &String) {
//     println!("{} {}", first, last)
// }
