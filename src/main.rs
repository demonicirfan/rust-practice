fn main() {
    // let s = String::from("hello");

    // takes_ownership(s.clone());
    // println!("{}", s);
    // let x = 5;
    // makes_copy(x);

    // let s1 = gives_ownership();
    let s2 = String::from("hello");
    takes_and_gives_back(s2.clone()); // s2 is cloned here
    let s3 = s2.clone();
    println!("{}", s2); // s2 is still valid here
    let s4 = s2;
    println!("{}", s3);
    println!("{}", s4);
}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }
fn takes_and_gives_back(a_string: String) {
    println!("{}", a_string)
}
