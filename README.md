#Ownership

- Ownership is a set of rules that governs how a rust program manages memory.
- In rust memory is managed through a system called ownership with a set of rules that the compiler checks.

## Rules of ownership
- Each value in rust has a variable thats called its owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped.

``` rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}

```

- When `s` comes into scope, it is valid.
- It remains valid until it goes out of scope.

## The `String` Type

 - Strings are a sequence of bytes, but they are stored in a heap-allocated buffer.
 - We can create a String from a string literal using the from function
  
``` rust
let s = String::from("hello");
```

**What is a namespace ?**

String::from -> Memory is allocated at runtime for the string.

``` rust
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no longer valid
}
```
In rust shallow copying is known as move ( An assignment in Rust is by default, a move. ) and deep copying is known as copy.
Any automatic copying can be assumed to be inexpensive in terms of runtime performance.
``` rust
    let s1 = String::from("hello");
    let s2 = s1;  // in this example s1 was moved into s2

    println!("{}, world!", s1);
```

what happens here is string is allocated on the heap and s1 is on the stack that is an object which contains information about the string such as pointer to the heap, capacity and length. When the variable s1 is moved to s2 the object on the stack is bitwise copied. This means that the pointer to the heap is copied and the capacity and length are copied. The buffer on the heap stays intact. This is indeed a move: it is now s2's responsibility to drop the heap buffer and s1 cant touch it.

The change of ownership is good because if access was allowed through both s1 and s2 then you will end up with two stack objects pointing to the same heap buffer.

which object should drop the buffer in this case? because that is not clear.Rust prevents this situation from arising at all.

Assignment is not the only operation which involves moves. Values are also moved when passed as arguments or returned from functions:
``` rust
    let s1 = String::from("hello");
    //s1 is first moved into print_ln's argument
    // and then moved into s2 when print_ln returns
    let s2 = print_ln(s1);  // in this example s1 was moved into s2

   fn print_ln(s1: String)-> String {
        println!("{}, world!", s1.len());
        s1 //s1 is moved out of the funciton
    }
```

### Moves
  - When a variable is declared with the `let` keyword, the compiler is required to create a copy of the value.
  - This is called a shallow copy.
  - The compiler will not copy the data, but rather, it will move the data into the new variable.
  - This is called a move.
  - The value that the variable holds is no longer valid.
  - The variable is still valid, but it holds a reference to the data that was moved.
 - This is called a move.

``` rust
let v:Vec<i32> = Vec::new();
let v1 = v;//v1 is the new owner
```

## Copies

```rust
let s: str = "hello";
let s1 = s;//s1 is the new owner
println('{}', s1)
```

In the above example the values are contained entierly in the stack. There is nothing to own on the heap; That is why it is ok to allow access throguh both s and s1 - they are completely independent copies.

Such types which do not own other resourc3es and can be bithwise copied are called ```copy``` types. They implement the copy marker trait. All primitive types like integers floats and characters are copy. Stucts or enums are not copy by default but you can derive the copy trait:

```rust
#[derive(Copy, Clone)] // you need to derive the Copy trait and the Clone trait
// for this to work all the members of the struct or enum must be copy themselves 
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
enum SignedOrUnsignedInt {
    Signed(i32),
    Unsigned(u32),
}
```

In general any type that implements Drop cannot be copy because drop is implemented by types which own resources and hence cannot be simply bitwise copied.But copy types should be trivially copyalbe. Hence, Drop and copy dont mix well.

Types that implement `copy`
- All the integer types, such as `u32`.
- The boolean type
- All the floating point types, such as `f64`
- The cahracter type, `char`
- Tuples, if they only contain types that also implement `Copy`, For example `(i32, i32)` implmenets `Copy` byt `(i32, String)` does not
- 


## Clone
When a value is moved, Rust does a shallow copy, but what if you want to create a deep copy like in c++? To allow that you need to implement the `Clone` trait. Then to make a deep `copy`, client code should call the `clone` method on the value.

``` rust
    let s1 = String::from("hello");
    let s2 = s1.clone(); // s2 is a clone of s1

    println!("{}, world!", s1);
```

Due to deep copying, both s1 and s2 are free to independently drop their heap buffers.

## References and Borrowing

 Ampersands ( `&` ) are used to represent references, they allow you to refer to some value without taking ownership of it

 ``` rust
 let s1 = String::from("hello");
 let len = calculate_length(&s1);
 ```

 the `&s1` syntax lets us create a reference that refers to the value of s1 but does not own it. Because it does not own it, The value it points to will not be dropped when the reference stops being used.

 ``` rust
 fn calculate_length(s: &string) -> usize {  // s is a reference to a String
  s.len()
 } // here s goes out of scope. But because it does not have ownership of what it refers to nothing happens
```

We call the action of creating a reference *borrowing*. 

Never try to modify a borrowed value

``` rust
fn main () {
    let s = String::from("hello");
    change(&s);
}

fn change(some_string: &String){
    some_string.push_str(", world!");
}
```

this will give an error ``` cannot borrow '*some_string' as mutable as it is behind a `&` reference```

### Mutable references

to fix the above code we will use a mutable reference

``` rust
fn main(){
    let s = String::from("hello");
    change(&mut s);
}

fn change (some_string: &mut String){ // change function will mutate the value it borrows
    some_string.push_str(", world");
}
```

- You can only have one mutable reference to a particular piece of data in a particular scope.

``` rust
let must s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time
```
We cannot have a immutable reference whiw we have an immutable one to the same value. Users of an immutable reference cdont expect the value to suddenly change out from under them however multiple references are allowed because no one who is just reading the data has the ability to affect anyone elses reading of the data. 

Not that a ference scope starts from where it is introduced and continues thourgh the last time that references is used.

```
 let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

This code compiles because the last usage of the immutable references the println! occus before the mutable reference is introduced.

Important points
- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.


## Slics

There are two types of strings in rust - String and &str.
Both String and &str are a grouping of characters(u8's);
Difference 
    - How they are stored in memory
    - How programmers uses
  
Strings
   - Heap
   - Mutable
(&str) string slick are complex
    - they are immutable ( exceptions )
    - often allocated on the stack, sometimes a heap reference sometimes embedded in the code
  - Can translate between String and str