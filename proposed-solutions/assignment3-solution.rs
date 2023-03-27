fn main () {
    let mut s = String::from("hello");

    { // putting the mutable borrow inside a separate code block limits its lifetime to the scope of the block
        let r1 = &mut s; // takes ownership of s
        println!("r1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s; // r1's lifetime is over, so still only one mutable borrow at a time

    println!("r2: {}", r2);
}
