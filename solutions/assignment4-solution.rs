fn main () {
    let mut s = String::from("hello");

    let r1 = &s; // immutable borrow (just a reference to s)
    let r2 = &s; // same here
    println!("{}, and {}", r1, r2); // this works, as we can have multiple _immutable_ references

    let r3 = &mut s; // this mutable borrow is ok, because there are no references to r1 & r2 after it

    println!("{}", r3); 

}