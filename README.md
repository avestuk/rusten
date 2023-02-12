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

You can allocate vectors using the builtin `vec!()` macro. 

```rust 
    // creates a vector [0, 1]
    let mut fib: Vec<u32> = vec![0, 1];
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

## Structs

You can build functions whose parameters match struct fields to avoid using the `field: value` syntax. 
```rust
# Declaring a struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

# Declaring a struct method
# self is short for self: Self where Self is a type alias for User inside the impl block
# Methods can take ownership so self can be owned, borrowed or mutably borrowed. 
impl User {
    fn isActive(&self) -> bool {
        self.active
    }
}

# A bit like in Go Rust will automatically de/reference an object when you call
a method on it. That means the below statements are synonymous
p1.distance(&p2);
(&p1).distance(&p2);

# Associated functions are functions that are associated with a certain type but are not Methods
impl User {
    fn active_user() -> User {
        User {
            active: true
        }
    }
}

# They are typically used as constructors and are accessed using `::` syntax
let active_user = User::active_user();

# Declaring a tuple struct, which is essentially just a named tuple but they are different types.
struct Color(i32,i32,i32);
struct Black(i32,i32,i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

# Create structs from other structs and override only specific fields. 
let user2 = User {
        email: String::from("another@example.com"),
        ..user1
};
```

The reason for specifying `user2 = User...` above is that we are actually
moving the values from user1 -> user2. We will no longer be able to access
`user1` after the data is moved. 

### Unless...

Remember that types that implement the `Copy` trait can be copied as they are
types with known sizes at compile time so they live on the stack. If the User
struct only had fields with types that implemented `Copy` then we could still
access `user1` after the copy. 

## Enums

```rust 

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

# Enums can embed structs
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

```
