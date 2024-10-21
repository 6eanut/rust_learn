enum Gender {
    Man,
    Woman,
}

impl Gender {
    fn check(&self) {
        match self {
            Gender::Man => {
                println!("man");
            }
            _ => {
                println!("woman")
            }
        }
    }
}

fn main() {
    let xiaoming = Gender::Man;
    let xiaomei=Gender::Woman;
    xiaoming.check();
    xiaomei.check();
}
