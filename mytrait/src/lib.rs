pub trait Dayin {
    fn dayin(&self);

    fn error(&self) {
        println!("error");
    }
}

#[derive(Debug)]
pub struct S1 {
    pub name: String,
    pub id: i32,
}

#[derive(Debug)]
pub struct S2 {
    pub name: String,
    pub checked: bool,
}

impl Dayin for S1 {
    fn dayin(&self) {
        println!("{:?}", self);
    }

    fn error(&self) {
        println!("error S1");
    }
}

impl Dayin for S2 {
    fn dayin(&self) {
        println!("{:?}", self);
    }
}

pub fn fn1<T: Dayin>(item: T) {
    item.dayin();
}

pub fn fn2<T>(item: T)
where
    T: Dayin,
{
    item.error();
}
