// 结构体可以通过impl来实现一些函数
// 当fn的第一个参数是self时，则该fn为结构体的方法
// 用法：struct object.method
// 当fn的第一个参数不是self时，则该fn为结构体的关联函数
// 用法：struct class::fn

#[derive(Debug)]
struct Rectangle {
    chang: u32,
    kuan: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.chang * self.kuan
    }

    fn changfangxing(chang0: u32, kuan0: u32) -> Rectangle {
        Rectangle {
            chang: chang0,
            kuan: kuan0,
        }
    }

    fn zhengfangxing(bianchang: u32) -> Rectangle {
        Rectangle {
            chang: bianchang,
            kuan: bianchang,
        }
    }

    fn can_hold(&self, another: Rectangle) -> bool {
        self.chang >= another.chang && self.kuan >= another.kuan
    }

    fn printtu(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let tu1 = Rectangle::changfangxing(20, 10);
    let tu2 = Rectangle::zhengfangxing(15);
    let tu3 = Rectangle::zhengfangxing(10);
    tu1.printtu();
    tu2.printtu();
    tu3.printtu();
    println!(
        "area, tu1:{} tu2:{} tu3:{}",
        tu1.area(),
        tu2.area(),
        tu3.area()
    );
    println!("tu1 hold tu2 : {}",tu1.can_hold(tu2));
    println!("tu1 hold tu3 : {}",tu1.can_hold(tu3));
}
