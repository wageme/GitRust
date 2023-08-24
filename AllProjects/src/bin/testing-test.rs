pub fn plus_one(value: u32) -> u32 {
    value + 1
}


#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn is_four() {
        assert_eq!(4, plus_one(3))
    }

    #[test]
    fn is_five() {
        assert_eq!(5, plus_one(4))
    }

    // This test should fail
    #[test]
    fn five_is_four() {
        assert_eq!(5, plus_one(3))
    }
}

fn main() { println!("Why are you running main? This is for tests :)"); }