fn main() {
    let s0=String::from("hello world");
    let s1=find_first_word(&s0);
    println!("{s1}");

    let a=[1,2,3];
    let b=&a[1..2];
    println!("{}",b[0]);
}

fn find_first_word(s:&str)->&str{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item ==b' '{
           return &s[..i];
        }
    }
    // used in the last line
    &s[..]
}

// slice (talked in array? such as String Array)
// like reference but only in array?
// use &str not & String