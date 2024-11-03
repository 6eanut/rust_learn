pub struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    closure: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    pub fn new(closure00: T) -> Cacher<T> {
        Cacher {
            closure: closure00,
            value: None,
        }
    }

    pub fn value(&mut self, input: i32) -> i32 {
        match self.value {
            None => {
                let v = (self.closure)(input);
                self.value = Some(v);
                v
            }
            Some(v) => {
                if v == input {
                    v
                } else {
                    let v = (self.closure)(input);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
}
