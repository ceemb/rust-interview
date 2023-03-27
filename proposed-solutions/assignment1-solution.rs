// Your assignment is to fix the errors in this code.

trait Speak {
    fn speak(&self);
}

// A bunch of races.
struct Human;
struct Orc;
struct HalfOrc;

impl Speak for Human {

    fn speak(&self) {
        println!("I'm a human.");
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
    let_unit_speak(&Human); // prints "I'm a human." (not explicitly instantiated like below)
    let_unit_speak(&Orc); // prints "I'm an orc."
    let_unit_speak(&HalfOrc); // prints "I'm half human and half orc.";
}