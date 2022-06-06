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

