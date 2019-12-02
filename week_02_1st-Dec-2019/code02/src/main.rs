struct Superman {
    name: String,
    mental_stability: String,
}

struct Batman {
    name: String,
    mental_stability: String,
}

struct Hulk {
    name: String,
    mental_stability: String,
}

struct Spiderman {
    name: String,
    mental_stability: String,
}

pub trait Power {
    fn power_score(&self)->u8 {
        50
    }
}

impl Power for Superman {
    fn power_score(&self)->u8{
        100
    }

}

impl Power for Batman {
    fn power_score(&self)->u8{
        80
    }
}

impl Power for Hulk {}

impl Power for Spiderman{}


fn notify<T: Power>(superhero: T) -> String {
    format!("{}", superhero.power_score())

}

fn notify_impl(superhero: impl Power)-> String {
    format!("{}", superhero.power_score())
}

fn notify_multiple<T: Power>(superhero1: T, superhero2: T) -> String {
    format!("{}, {}", superhero1.power_score(),superhero2.power_score())
}

fn notify_multiple_impl(superhero1: impl Power, superhero2: impl Power) -> String {
    format!("{}, {}", superhero1.power_score(),superhero2.power_score())
}



fn main() {
    let superman = Superman {name: String::from("Superman"), mental_stability: String::from("Stable")};
    let batman = Batman {name: String::from("Batman"), mental_stability: String::from("In-stable")};
    let hulk = Hulk {name: String::from("Hulk"), mental_stability: String::from("Stable")};
    let spiderman = Spiderman {name: String::from("Spiderman"), mental_stability: String::from("In-stable")};
    
    println!("Superman power: {}",superman.power_score())   ;
    println!("Batman power: {}",batman.power_score());
    println!("Hulk power: {}",hulk.power_score())   ;
    println!("Spiderman power: {}",spiderman.power_score());

    //println!("{}",notify(superman));
    //println!("{}",notify_impl(batman));

    println!("{}",notify_multiple(superman, batman));
    //println!("{}",notify_multiple_impl(superman, batman));
}
