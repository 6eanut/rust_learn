use std::collections::btree_map::Range;

// const NAME_DAXIE : type = value;
// const MAX_NUMS: i32 = 5;
fn main() {
    second();
}

fn second() {
    // loop-break
    let mut x = 5;
    let result = loop {
        if x == 10 {
            break x;
        }
        x = x + 1;
    };
    println!("result = {}", result);

    // while
    while x < 20 {
        x = x + 1;
        println!("ok");
    }

    // for
    let array00 = [1, 2, 3, 4, 5];
    for element in array00.iter() {
        println!("{}", element);
    }
    // rev reverse
    for number in (1..=3).rev() {
        println!("number: {}", number);
    }
}

// fn first() {
//     // const
//     println!("{}", MAX_NUMS);

//     // tuple
//     let mytuple = (3, 2.1, 'a', true);
//     let (x, y, z, m) = mytuple;
//     println!(
//         "mytuple: {} {} {} {}",
//         mytuple.0, mytuple.1, mytuple.2, mytuple.3
//     );
//     println!("x={}, y={}, z={}, m={}", x, y, z, m);

//     // array
//     let myarray = [1, 2, 3, 4];
//     let myarray0 = [3; 5];
//     println!("{}, {}", myarray[0], myarray0[2]);

//     // function
//     let b = 32;
//     // b is an argument
//     println!("plus_six(a)={}", plus_six(b));

//     // if else
//     let conditon = -1;
//     if conditon < 0 {
//         println!("less 0");
//     } else if conditon > 0 {
//         println!("greater 0");
//     } else {
//         println!("equal 0");
//     }
//     //shadow
//     let conditon = false;
//     println!("if's result {}", if conditon { 3 } else { 5 });
// }

// // a is a parameter
// fn plus_six(a: i32) -> i32 {
//     a + 6
// }
