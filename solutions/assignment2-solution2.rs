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
        if input.day == self.day {
            return true;
        }
        false
    }

    pub fn update_event(mut event: Event) { // function now uses (less idiomatic) reference to event2 instead of "self"
        println!("update-event");
        event.day = event.day + 1;
        println!("{} was moved to {}/{} {}", event.name, event.day, event.month, event.year); // print moved into scope where we have ownership of event2
    }

}

pub fn main() {
    let event1 = Event::new("Pac-12 Championship".into(), 1, 12, 2017); 
    let event2 = Event::new("Group Project Meeting".into(), 1, 12, 2017);
    if event1.has_conflict(&event2) {
        Event::update_event(event2); // called through struct definition instead of going through event2
    } else {
        println!("No conflicts");
    }
}