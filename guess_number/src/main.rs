// prelude
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Range 1..5, 1..=5, 1.., ..5

fn main() {
    let answer = rand::thread_rng().gen_range(1..10);
    println!("guess a number(1..10)");
    loop {
        // immutable mutable
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("read failed");
        let number: i32 = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("please enter a number! guess angin:");
                continue;
            }
        };
        // println!("the number you guess: {}", number);
        // println!("answer is :{}", answer);
        match number.cmp(&answer) {
            // three arms
            Ordering::Less => println!("too small! guess again:"),
            Ordering::Greater => println!("too big! guess again:"),
            Ordering::Equal => {
                println!("that's right");
                break;
            }
        }
    }
}
