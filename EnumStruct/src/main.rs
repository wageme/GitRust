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

    fn display(&self) {
        println!("Hello, {}!\nYour stats are-", self.Class);
        println!("\tHP: {}\n\tEffective HP: {}\n", self.HP, self.effective_health());
    }
}

fn main() {
    let ninja = Hero {
        Class: "Ninja".to_string(),
        HP: 20.0,
        Armor: Material::Leather,
    };

    let warrior = Hero {
        Class: "Warrior".to_string(),
        HP: 25.0,
        Armor: Material::Iron,
    };

    let cleric = Hero {
        Class: "Cleric".to_string(),
        HP: 15.0,
        Armor: Material::Gold,
    };

    ninja.display();
    warrior.display();
    cleric.display();
}
