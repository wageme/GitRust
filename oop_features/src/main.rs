use oop_features::my_lib;

// Dont forget to use 'cargo run' to make sure this works

fn main() {
    println!("Hello, world!");
    let mut my_collection: my_lib::AveragedCollection = my_lib::AveragedCollection::new();
    my_collection.add(2);
    my_collection.add(7);
    print!("average: {}", my_collection.average());
}

