// enum和struct有点像
// 都可以通过impl来实现方法
// 都可以在里面定义各种类型
fn main() {
    let jiakai=Person::sex(Gender::man);
    println!("{:?}",jiakai);
}

#[derive(Debug)]
enum Gender {
    man,
    woman,
}

#[derive(Debug)]
enum Person {
    name(String),
    sex(Gender),
}
