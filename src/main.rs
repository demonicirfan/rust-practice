fn main() {
    // let s = String::from("hello");

    // takes_ownership(s.clone());
    // println!("{}", s);
    // let x = 5;
    // makes_copy(x);

    // let s1 = gives_ownership();
    // let s2 = String::from("hello");
    // takes_and_gives_back(s2.clone()); // s2 is cloned here
    // let s3 = s2.clone();
    // println!("{}", s2); // s2 is still valid here
    // let s4 = s2;
    // println!("{}", s3);
    // println!("{}", s4);
   // let mut s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    // println!("The length of {} is {}", s2, len)
    // let _r1 = &s1; // s1 is valid here and this is a reference to it ( mutable borrow)
    // let _r2 = &s1; // s1 is valid here and this is a reference to it ( mutable borrow)
    // let r4 = s1; // error: cannot borrow `s1` as mutable because it is also borrowed as immutable
    // println!("{}{}", _r1, _r2);
    // let _r3 = &mut s1;
    // println!("{} ", r4);
    let example_str: &str = "howdey";
    let exmaple_string: String = String::from("hello");

    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "some hardcoded string".to_string();

    let string_from_hardcoded = String::from("some hardcoded data");
    let string_from_str_var = String::from(example_str);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
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
