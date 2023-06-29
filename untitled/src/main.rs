use rand::Rng;

fn main() {

    // Vectors of Greetings and Subjects
    let greetings = vec!["Hello", "Hi", "Greetings", "Salutations", "Hola", "Salut", "Allo", "Hallo"];
    let subjects = vec!["World", "Sir", "Madam", "Captain"];

    /*
    let greet_index = rand::thread_rng().gen_range(0..greetings.len());
    let subject_index = rand::thread_rng().gen_range(0..subjects.len());
*/

    // Get random greeting and subject
    let greeting = greetings[rand::thread_rng().gen_range(0..greetings.len())];
    let subject =  subjects[rand::thread_rng().gen_range(0..subjects.len())];

    println!("{}, {}!", greeting, subject);

    if greeting == "Hello" && subject == "World" {
        println!("Working as intended :)")
    }
}
