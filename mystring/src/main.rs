// string

fn main() {
    // create and init
    let mut s1 = String::new();
    let mut s2 = "hello, to_string".to_string();
    let mut s3 = String::from("hello, from");
    let s4 = String::from("check");
    let s7=&s4[2..5];

    // write
    s1.push('y');
    s2.push_str(" ok");
    let s5 = s3 + &s4;
    let s6=format!("{}-{}",s5,s2);

    println!("{}", s2);
    println!("{}", s1);
    println!("{}", s5);
    // error: println!("{}", s3);
    println!("{}", s6);
    println!("{}", s7);

    // error: s6[3] buzhichi suoyin fangwen
}
