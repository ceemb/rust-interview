// Your assignment is to fix the errors. Examples are from: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// This assignment is just meant to illustrate the concept of ownership and lifetimes, so you can look at the article and the 
// solution if you can't immediately solve it. The emphasis is on trying to understand the concept. 
    
fn main () {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

}
