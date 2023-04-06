// NOTE: You don't need to fix these errors, 8just try to understand why they occur. 
// Examples are from: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

fn main () {
    let mut s = String::from("hello");

    let r1 = &s; // immutable borrow (just a reference to s)
    let r2 = &s; // same here
    println!("{}, and {}", r1, r2); // this works, as we can have multiple _immutable_ references

    let r3 = &mut s; // errors can be fixed without changing this line

    println!("{}, {}, and {}", r1, r2, r3); // errors can be fixed by changing this line
    // NOTE: You don't need to fix these errors, just try to understand why they occur.

}