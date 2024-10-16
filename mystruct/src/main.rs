fn main() {
    let p1 = Person {
        name: String::from("jack"),
        age: 32,
    };

    let name = String::from("tom");
    let p2 = Person0 {
        name: &name,
        age: 21,
    };
    println!("{}, {}", p1.name, p1.age);
    println!("{}, {}", p2.name, p2.age);

    struct Color(i32, i32, i32);
    let color = Color(10, 20, 30);
    println!("{} {} {}", color.0, color.1, color.2);

    let re=Rectangle{
        wide:20,
        long:14,
    };
    println!("{}",area(&re));
    println!("{:#?}",re);
}
struct Person {
    name: String,
    age: u32,
}

// lifetime
struct Person0<'a> {
    name: &'a str,
    age: u32,
}

#[derive(Debug)]
struct Rectangle {
    wide: u32,
    long: u32,
}

fn area(rec: &Rectangle) -> u32 {
    rec.long * rec.wide
}
