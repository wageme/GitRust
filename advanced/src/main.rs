extern "C" {
    fn abs(input: i32) -> i32;
}



// We can create raw pointers in safe rust, but not dereference (unless in unsafe rust)

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let food = String::from("Pizza");  // A food variable of type string
    let ptr: *const String = &food as *const String;    // A pointer variable, with the name ptr, that stores the address of food
    
    // Output the value of food (Pizza)
    println!("{}", food);

    // Output the memory address of food (0x6dfed4)
    println!("{:?}", &food as *const String);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
