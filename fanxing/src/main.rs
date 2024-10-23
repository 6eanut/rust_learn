// function
fn sum<T>(a: &T, b: &T) {
    println!("hello");
}
// struct
struct Person<T, U> {
    age: T,
    name: U,
}

// method
impl<T, U> Person<T, U> {
    fn f1(self) {
        println!("check");
    }
    fn f2<V, W>(self, other: Person<V, W>) {
        println!("check2");
    }
}

impl Person<i32, f32> {
    fn f3(self) {
        println!("check3");
    }
}

fn main() {
    println!("Hello, world!");
}
