use rand::Rng;

struct cash_stats {
    multiplier: f64,
    bid_fraction: f64,
    cash: f64,
    chance: f64,
    generation: i32,
}

impl cash_stats {
    fn generate_bid_text(&self){
        println!("GEN: {}", self.generation);
        println!("Lets bid {}% of our money (${})", &self.bid_fraction * 100.0, &self.cash * &self.bid_fraction);
        println!("We have a {}% chance of winning {}x our bid (${})", &self.chance * 100.0, &self.multiplier, &self.cash * &self.bid_fraction * &self.multiplier);
    }

    fn generates(&mut self){
        // get the randomizer
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..100);
        // If value exceeds chance amount, then it's a win
        if value >= (100.0 * self.chance) as i32 {
            println!("YOU WIN!"); // Winning the bet
            self.cash = self.bid_fraction * self.cash * (self.multiplier - 1.0);
        } else { // Losing the bet
            println!("Womp womp...");
            self.cash = self.cash * (1.0 - self.bid_fraction)
        }

        self.generation += 1;
        println!("\n");
        // println!("Number: {}", rng.gen_range(0..100));
    }
}





fn main() {

    let mut stats = cash_stats {
        multiplier: 2.0,
        bid_fraction: 0.5,
        cash: 100.0,
        chance: 0.5,
        generation: 0,
    };

    println!("Hello! You are starting with {} dollars", stats.cash);
    stats.generate_bid_text();
    stats.generates();

    stats.generate_bid_text();
}
