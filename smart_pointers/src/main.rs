/*

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
*/
use std::ops::Deref;


// Example of struct that copies functionality of Box<T>
struct MyBox<T>(T); // Contains just a generic type parameter T


impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { // Constructor that returns a MyBox containing any variable
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//-----------------------------------------------------
// #[derive(Default)] // Use this if you don't want to make your own default
pub struct Second {
    value: u64
}

impl Second {
    /// Returns the value in seconds.
    pub fn value(&self) -> u64 {
        self.value
    }
}

// Your own default
impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}

//-----------------------------------------------------

// Custom smart pointer that prints something whenever dropped
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

// Custom smart pointer that prints something whenever dropped

// Reference counter
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;



fn main() {
    
    //let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // let b = Box::new(5);
    // println!("b = {}", b);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

}


#[test]
fn drop_test() {
    let _d = CustomSmartPointer {data: String::from("Test Text")};
    println!("Custom Smart Pointer created.")
}





#[test]
fn rctest(){ // Testing "Reference Counted" smart pointer
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}