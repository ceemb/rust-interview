trait Speak {
    fn speak(&self);
}

// A bunch of races.
struct Human {
    name: String
}
struct Orc;
struct HalfOrc;

// the below block lets us instantiate "human" on line 56
impl Human {
    fn new(name: String) -> Self {
        Self {
            name: name, // this sets your given name to the instance of Human

        }
    }
}

impl Speak for Human {

    fn speak(&self) {
        println!("I'm a human.");
        println!("My name is {}", self.name) // add this line when you have implemented the name field for humans
    }
}

impl Speak for Orc {
    fn speak(&self) {
        println!("I'm an orc.");
    }
}

impl Speak for HalfOrc {
    fn speak(&self) {
        println!("I'm half human and half orc.");
    }
}

/// Free function that takes a reference to any object that implements Speak.
fn let_unit_speak(unit: &impl Speak) {
    unit.speak();
}

fn main() {
    // prints "I'm a human." (not explicitly instantiated like below)
    // The line below should make a human (different from the instance on line 56) 
    // introduce themselves and their name.
    let_unit_speak(&Human); 

    let_unit_speak(&Orc); // prints "I'm an orc."
    let_unit_speak(&HalfOrc); // prints "I'm half human and half orc."

    let human = Human::new(); // instantiate human
    human.speak(); // here, the instance "human" introduces their name
}