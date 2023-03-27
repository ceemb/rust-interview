trait Speak {
    fn speak(&self);
}

// A bunch of races.
struct Human {
    name: String
}
struct Orc;
struct HalfOrc;

impl Human {
    fn new(name: String) -> Self {
        Self {
            name: name,

            // it's also ok to write just "name" because the parameter name and the struct name are exactly the same:
            // name // this works instead of line 15 (and reduces boilerplate)

        }
    }
}

impl Speak for Human {

    fn speak(&self) {
        println!("I'm a human.");
        println!("My name is {}", self.name)
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
    let_unit_speak(&Human {name: "Kalle".to_string()}); // prints "I'm a human." (not explicitly instantiated like below)
    let_unit_speak(&Orc); // prints "I'm an orc."
    let_unit_speak(&HalfOrc); // prints "I'm half human and half orc."

    let human = Human::new("Valle".to_string()); //instantiate human
    human.speak();
}