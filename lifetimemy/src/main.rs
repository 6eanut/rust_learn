fn main() {
    let s1 = String::from("hello");
    let s2 = "goodbye";
    let s3 = longer(s1.as_str(), s2);
    println!("longer is {}", s3);
}

// return type contains a borrow value
// the signature must say the lifetime
fn longer<'a>(a1: &'a str, a2: &'a str) -> &'a str {
    if a1 > a2 {
        a1
    } else {
        a2
    }
}
