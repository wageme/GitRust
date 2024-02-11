/*
What does this do:
- Goku struct which contains a Transformation struct
- Can power up, power down, power off, and read power level
 */


struct Goku {
    state: Option<Box<dyn Transformation>>,
}

impl Goku {
    pub fn new() -> Goku {
        Goku {
            state: Some(Box::new(Base {})),
        }
    }

    pub fn powerdown(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.powerdown());
        }
    }

    pub fn powerup(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.powerup());
        }
    }

    pub fn poweroff(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.poweroff());
        }
    }

    fn pwrlevel(&self) {
        self.state.as_ref().unwrap().pwrlevel();
    }

}

trait Transformation {
    fn powerup(self: Box<Self>) -> Box<dyn Transformation>;
    fn poweroff(self: Box<Self>) -> Box<dyn Transformation>;
    fn powerdown(self: Box<Self>) -> Box<dyn Transformation>;
    fn pwrlevel(&self);
}

struct Base {}

struct SSJ {}

struct SSG {}

struct SSGSS {}


impl Transformation for Base {
    fn powerup(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(SSJ {})
    }
    fn poweroff(self: Box<Self>) -> Box<dyn Transformation> {
        self
    }
    fn powerdown(self: Box<Self>) -> Box<dyn Transformation> {
        self
    }
    fn pwrlevel(&self) {
        println!("Pwrlevel: 10");
    }
}



impl Transformation for SSJ {
    fn powerup(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(SSG {})
    }
    fn poweroff(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(Base {})
    }
    fn powerdown(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(Base {})
    }
    fn pwrlevel(&self) {
        println!("Pwrlevel: 1,000");
    }
}

impl Transformation for SSG {
    fn powerup(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(SSGSS {})
    }
    fn poweroff(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(Base {})
    }
    fn powerdown(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(SSJ {})
    }
    fn pwrlevel(&self) {
        println!("Pwrlevel: 100,000");
    }
}

impl Transformation for SSGSS {
    fn powerup(self: Box<Self>) -> Box<dyn Transformation> {
        self
    }
    fn poweroff(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(Base {})
    }
    fn powerdown(self: Box<Self>) -> Box<dyn Transformation> {
        Box::new(SSG {})
    }
    fn pwrlevel(&self) {
        println!("Pwrlevel: 10,000,000");
    }
}


fn main() {
    let mut goku = Goku::new();
    goku.pwrlevel();
    goku.powerup();
    goku.pwrlevel();
    goku.powerup();
    goku.pwrlevel();
    goku.powerup();
    goku.pwrlevel();
    goku.powerdown();
    goku.pwrlevel();
    goku.poweroff();
    goku.pwrlevel();

}