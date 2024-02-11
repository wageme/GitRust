pub trait Animal {
    fn noise(&self) {
        println!("...");
    }
}

impl Animal for Dog {
    fn noise(&self) {
        println!("Bark!");
    }
}

impl Animal for Cat {
    fn noise(&self) {
        println!("Meow!");
    }
}

impl Animal for Giraffe {}

pub struct Dog {
    pub name: String,
}

pub struct Cat {
    pub name: String,
}

pub struct Giraffe {

}

fn main() {
    let mut pets: Vec<Box<dyn Animal>> = Vec::new();

    pets.push(Box::new(Dog {name: "Cupcake".to_string()}));
    pets.push(Box::new(Cat {name: "Cupcake".to_string()}));
    pets.push(Box::new(Giraffe {}));
    //pets.push(Box::new(Animal {})); // This does not work
    
    for pet in pets.iter() {
        pet.noise();
    }
}