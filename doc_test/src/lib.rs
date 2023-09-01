//! # lib
//!
//! 'lib' is an example use of a library containing functions of 
//! any sort

pub mod my_module {

    /// Compliments the user on their appearance
    /// 
    /// # Examples
    /// 
    /// ```
    /// look_in_mirror(); // Observe self
    /// compliment(); // Respond appropriately
    /// ```
    pub fn compliment() {
        println!("You're pretty :)")
    }

    /// Adds two i32 numbers
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}