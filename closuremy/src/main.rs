use closuremy;
fn main() {
    let closure00 = |num: i32| -> i32 {
        let nums = num + 1;
        println!("{}", nums);
        num
    };
    let num = closure00(2);
    println!("{}", num);

    let closure_example = |num| num + 1;
    let mut cacher = closuremy::Cacher::new(closure_example);
    let v = cacher.value(20);
    let v2 = cacher.value(33);
    println!("{}, {}", v, v2);

    let check = vec![3, 2, 1];
    let closure01 = move |num| num == check;
    // println!("{}", check);
    let boo1 = closure01(vec![23, 2, 1]);
    println!("{}", boo1);
}
