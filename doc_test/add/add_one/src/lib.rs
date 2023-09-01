/// Adds one to an i32 value
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_one(3));
    }
}

// C:\Users\Wage Me\.cargo\bin\rg.exe

// cargo --list