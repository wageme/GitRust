/*
    You have a 60% chance to double your bid amount and a 40% chance to win
    Easy to assume you'd win infinite money if you kept going, but not quite so...
*/


use rand::Rng;
use std::io::{stdin, stdout, Read, Write};

struct CashStats {
    multiplier: f64,
    bid_fraction: f64,
    cash: f64,
    chance: i32,
    generation: i32,
}

impl CashStats {
    fn generate_bid_text(&self){
        println!("GEN: {} | ${}", self.generation, self.cash);
        println!("Lets bid {}% of our money (${})", &self.bid_fraction * 100.0, &self.cash * &self.bid_fraction);
        println!("We have a {}% chance of winning {}x our bid (${})", self.chance, self.multiplier, self.cash * self.bid_fraction * self.multiplier);
    }

    fn generates(&mut self){
        // get the randomizer
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..100);
        // If value exceeds chance amount, then it's a win
        if value <= self.chance {
            println!("\nYOU WIN!"); // Winning the bet
            self.cash = self.cash + self.bid_fraction * self.cash * (self.multiplier - 1.0);
        } else { // Losing the bet
            println!("\nWomp womp...");
            self.cash = self.cash * (1.0 - self.bid_fraction)
        }

        self.generation += 1;
        println!("\n");
        // println!("Number: {}", rng.gen_range(0..100));

    }
}


fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}


fn main() {

    let mut stats = CashStats {
        multiplier: 2.0, // How the money you've bet will be multiplied if you win
        bid_fraction: 0.5, // Fraction of your currency that you're betting
        cash: 100.0, // Cash you start with
        chance: 60, // Chance to win in %
        generation: 0,
    };

    println!("Hello! You are starting with {} dollars", stats.cash);

    while true {
        stats.generate_bid_text();

        pause(); // Press enter to proceed

        stats.generates();
    }


}