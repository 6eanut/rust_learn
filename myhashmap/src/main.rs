use std::collections::HashMap;

fn main() {
    let mut h1 = HashMap::new();
    h1.insert(String::from("rm"), 10);
    h1.insert(String::from("bc"), 20);

    for (k, v) in &h1 {
        println!("{}-{}", k, v);
    }

    let score = h1.get(&String::from("rm"));
    match score {
        Some(s) => {
            println!("score : {}", s);
        }
        None => {
            println!("no team");
        }
    }
    let score2 = h1.get(&String::from("ac"));
    match score2 {
        Some(s) => {
            println!("score : {}", s);
        }
        None => {
            println!("no team");
        }
    }

    h1.insert(String::from("rm"), 20);
    println!("{:?}", h1);
    let have = h1.entry(String::from("bc"));
    have.or_insert(30);
    println!("{:?}", h1);
    let have = h1.entry(String::from("yu"));
    have.or_insert(30);
    println!("{:?}", h1);

    let s1 = String::from("hello world hello rust");
    let mut h2 = HashMap::new();
    for word in s1.split_whitespace() {
        let count = h2.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",h2);
}
