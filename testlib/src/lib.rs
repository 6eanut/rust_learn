pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Guess {
    number: i32,
}
pub fn check_number(num: i32) -> Guess {
    if num > 100 {
        panic!("nooooooooo");
    } else if num < 0 {
        panic!("yesssssss");
    }
    Guess { number: num }
}
impl Guess {
    pub fn print_guess(&self) {
        println!("{}", self.number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test02() {
        let ressult = add(3, 4);
        let message = "naqi";
        assert_ne!(ressult, 8, "check one two {}", message);
    }

    #[test]
    #[should_panic(expected = "no")]
    fn test03() {
        let a: i32 = 200;
        let g = check_number(a);
        g.print_guess();
    }

    #[test]
    #[should_panic(expected = "no")]
    fn test04() {
        let a: i32 = 200;
        let g = check_number(a);
        g.print_guess();
    }

    #[test]
    #[ignore = "1111111111111"]
    fn test05() -> Result<(), String> {
        if 2 + 2 != 4 {
            Ok(())
        } else {
            Err(String::from("err"))
        }
    }

    // -- --show-output
    // -- --test-threads
    // -- --ignored
}
