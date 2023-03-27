struct Event {
    name: String,
    day: u8,
    month: u8,
    year: u32
}

impl Event {
    pub fn new(name: String, day: u8, month: u8, year: u32) -> Event {
        Event { name, day, month, year }
    }

    pub fn has_conflict(&self, input: &Event) -> bool {
        if input.day == self.day && input.month == self.month && input.year == self.year {
            return true;
        }
        false
    }

    pub fn update_event(&mut self) { // mutable reference of event2 as input
        println!("update-event");
        self.day = self.day + 1; 
        //println!("{} was moved to {}/{} {}", self.name, self.day, self.month, self.year); // alternative to row 33
    }

}

pub fn main() {
    let event1 = Event::new("Pac-12 Championship".into(), 1, 12, 2017);
    let mut event2 = Event::new("Group Project Meeting".into(), 1, 12, 2017); // event2 is mutable
    if event1.has_conflict(&event2) {
        event2.update_event(); 
        println!("{} was moved to {}/{} {}", event2.name, event2.day, event2.month, event2.year);
    } else {
        println!("No conflicts");
    }
}