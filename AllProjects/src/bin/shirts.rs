
// Enumerate the different shirt colours
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour{
    Red,
    Blue,
}

// Struct contains vector of shirt colour Enums
struct Inventory {
    shirts: Vec<ShirtColour>
}

impl Inventory { // impl gives functions to the struct "Inventory"
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour { // 2 Parameters, returns ShirtColour (Enum)
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    // Returns most stocked shirt colour
    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;

        // Iterate through shirts in the Inventory Struct
        for colour in &self.shirts {
            match colour { // Add to counter depending on colour
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }
        // Return the shirt colour that is of higher quantity (most stocked)
        if num_red > num_blue {
            ShirtColour::Red
        } else {
            ShirtColour::Blue
        }
    }
}


fn main(){
    // Initialize store with an inventory of 2 blue and 1 red shirt
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue],
    };

    let user_pref1 = Some(ShirtColour::Red); // User prefers red shirts, Some is from Option enum
    //let user_pref1 = None; // Since user has no preference, he gets the most stocked option
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
}