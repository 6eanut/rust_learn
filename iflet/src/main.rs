fn main() {
    let xiaoming = Gender::Man;
    if let Gender::Man = xiaoming {
        println!("man");
    } else {
        println!("woman");
    }
    let xiaohong = Gender::Woman;
    if let Gender::Man = xiaohong {
        println!("man");
    } else {
        println!("woman");
    }
    if let xiaohong = Gender::Man {
        print!("yes");
    } else {
        print!("no");
    }
}

enum Gender {
    Man,
    Woman,
}
// 需要注意 iflet的後面是跟了一個等式，這個等式的左邊必須是定植，不能是變量，否則會出錯
