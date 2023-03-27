// Your assignment is to fix the errors. Examples are from: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    
fn main () {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

}
