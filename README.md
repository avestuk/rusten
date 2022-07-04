# README

A place to sharpen skills in case they get rusty. 

We follow the [good book](https://doc.rust-lang.org/book/) 'ere 

# Notes

## Cargo

- `Cargo new` creates a standard directory structure and a git repo   
- `Cargo build` builds a binary - it'll land in the `target/debug` directory
- `Cargo build --release` builds a binary for release - it takes longer but adds optimizations
- `Cargo run` builds and runs the binary
- `Cargo check` verifies that the binary compiles

## Assorted

- `expect()` will panic with a custom message 
- `match` will let us handle the error instead

### Shadowing

Variables can be shadowed - i.e declare the same variable multiple times, with
different values (even different types!). However you cannot shadow a `mut`
variable with a different type. The variables are scoped. 
```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {12}", x);
    }

    println!("The value of x is: {6}", x);

    let x = "banana"


    println!("The value of x is: {banana}", x);
}
```

## Tuples

Tuples have a fixed length and can contain multiple types. 
```rust 
let t: (i32, f64, u8) = (500, 2.4, 8);

# Tuple values can be accessed by index
let five_hundred = x.0; 

# Tuples can also be deconstructed
let five_hundred, two_point_four, eight = t; 

```

## Arrays 

Arrays have a fixed length and can only contain a single type. Arrays are
allocated on the stack rather than the heap. A vector is like an array except it
can grow/shrink in size.  

```rust
# You can declare the type and size of the array using type hinting inside
# square brackets
 let a:[i32: 3] = [4, 100, 10];

# You can fill an array using value; size syntax as follows
let a = [3; 5];
# This is equivalent to 
let a = [3, 3, 3, 3, 3];
```

## Statements and Expressions

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resulting value. Let’s look at some examples.

```rust
let y = 6; # Expression

    let y = { # Everything inside the {} is an expression.. so expressions can be part of statements. Consider that let y = 4 is a statement!
        let x = 3;
        x + 1 # This line does not have a semicolon as it is an expression. If it did it would not return a value! 
    };

```

