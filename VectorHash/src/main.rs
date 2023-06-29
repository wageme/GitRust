use std::collections::HashMap;

fn printhash(book: &HashMap<String, u32>){
    for (key, value) in book {
        println!("{}: {}", key.to_string(), value);
    }
}

fn main() {
    // Phonebook hashmap (4 digit numbers)
    let mut phonebook = HashMap::new();

    phonebook.insert(String::from("Michael"), 2943);
    phonebook.insert(String::from("Steven"), 9261);
    phonebook.insert(String::from("Ophelia"), 4726);
    phonebook.insert(String::from("Sarah"), 1129);

    printhash(&phonebook);

}
