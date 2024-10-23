// vector string hashmap
// in heap

#[derive(Debug)]
enum Kindinput {
    Myint(i32),
    Mystring(String),
    Myfloat(f32),
}

fn main() {
    // vector
    // Vec::new()
    let mut v1 = Vec::new();
    // macro vec![]
    let v2 = vec![1, 2, 3];
    // push
    v1.push(32);
    // bianli vector
    for ele in &v2 {
        println!("{}", ele);
    }
    // read
    println!("{}", v2[1]);
    match v2.get(2) {
        Some(third) => {
            println!("{}", third);
        }
        None => {
            println!("error");
        }
    }
    match v2.get(4) {
        Some(third) => {
            println!("{}", third);
        }
        None => {
            println!("error");
        }
    }

    let v3 = vec![
        Kindinput::Myint(2),
        Kindinput::Mystring(String::from("jack")),
        Kindinput::Myfloat(3.2),
    ];
    for ele in &v3{
        println!("{:?}",ele);
        // error: println!("{}",ele);
    }
}
