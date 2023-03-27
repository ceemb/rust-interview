// Your assignment is to fix the errors. Examples are from: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main () {
    let mut s = String::from("hello");

    let r1 = &s; // immutable borrow (just a reference to s)
    let r2 = &s; // same here
    println!("{}, and {}", r1, r2); // this works, as we can have multiple _immutable_ references

    let r3 = &mut s; // don't change this line

    println!("{}, {}, and {}", r1, r2, r3); // change this line

}