// Rust does not have inheritance or classes. Instead, similar functionality is achieved with structs and traits.
// Your assignment is to make this code compile , as well as make sure the Human introduces themselves with a name in line 54 and line 60.

// Documentation on structs: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
// Documentation on using methods on structs: https://doc.rust-lang.org/book/ch05-03-method-syntax.html 

trait Speak {
    fn speak(&self);
}

// A bunch of races.
struct Human {
    name: String
}
struct Orc;
struct HalfOrc;

// the below block lets us instantiate "human" on line 62
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
        // add this line when you have implemented the name field for humans
        //println!("My name is {}", self.name) 
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

/// Free function that takes a reference to any object that implements Speak (line 7).
fn let_unit_speak(unit: &impl Speak) {
    unit.speak();
}

fn main() {
    // line 54 prints "I'm a human." (without a "let" statement like on line 62)
    // The line below should make a human introduce themselves and their name.
    let_unit_speak(&Human); 

    let_unit_speak(&Orc); // prints "I'm an orc."
    let_unit_speak(&HalfOrc); // prints "I'm half human and half orc."

    let human = Human::new(); // instantiate human (not the same human an on line 57)
    human.speak(); // here, the instance "human" introduces themselves with their name (uncomment line 33 when you are ready)
}