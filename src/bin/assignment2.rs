// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;


struct Event {
    name: String,
    day: u8,
    month: u8,
    year: u32
}

// you need to add any arguments and return values to the functions below 
// (you can use existing functions as inspiration)
impl Event {
    pub fn new(name: String, day: u8, month: u8, year: u32) -> Event {
        Event { name, day, month, year }
    }

    pub fn has_conflict() { 
        //your code here
    }

    // move event one day forward 
    // (you can assume that it's not at the end of a month)
    pub fn update_event() { 
        //your code here
    }

}

// it's ok to make edits below 
// (a solution is possible with less than 10 characters edited below))
pub fn main() {  
    let event1 = Event::new("Pac-12 Championship".into(), 1, 12, 2017); // try changing this to a non-confligting date as a test
    let event2 = Event::new("Group Project Meeting".into(), 1, 12, 2017);
    if event1.has_conflict(event2) {
        event2.update_event();
        println!("{} was moved to {}/{} {}", event2.name, event2.day, event2.month, event2.year);

    } else {
        println!("No conflicts");
    }
}