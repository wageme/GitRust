enum Material {
    None,
    Leather,
    Chainmail,
    Iron,
    Gold,
    Platinum,
}

struct Hero {
    Class: String,
    HP: f32,
    Armor: Material
}

impl Hero {
    fn effective_health(&self) -> f32 {
        match self.Armor {
            Material::None => self.HP,
            Material::Leather => self.HP * 1.5,
            Material::Chainmail => self.HP * 2.0,
            Material::Iron => self.HP * 2.5,
            Material::Gold => self.HP * 3.0,
            Material::Platinum => self.HP * 3.5,
        }
    }
}

fn main() {
    let character = Hero {
        Class: "Ninja".to_string(),
        HP: 20.0,
        Armor: Material::Leather,
    };

    println!("Hello, {}!\nYour stats are-", character.Class);
    println!("\tHP: {}\n\tEffective HP: {}\n", character.HP, character.effective_health());
}
