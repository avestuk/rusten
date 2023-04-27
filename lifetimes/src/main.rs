fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result1 = longest(string1.as_str(), string2); // Result is a reference to string1 OR string2
    println!("The longest string is {}", result1); // The variable that is not referenced can be
                                                   // dropped - Hence the need for lifetimes

    let string3 = String::from("long string is long");

    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result2); // string2 & result are valid until the
                                                       // end of the inner scope - while
                                                       // string1 is valid until the end
                                                       // of the outer scope.
    }

    let string5 = String::from("long string is long");
    let result3;

    {
        let string6 = String::from("xyz");
        result3 = longester(string5, string6);
        // string6 is only valid in the inner scope so the lifetime of result3 exceeds the lifetime
        // of string6. This violates the contract that the reference returned by longest() will
        // live at least as long as lifetime 'a
    }
    println!("The longest string is {}", result3);
}

// We want the signature to express the following constraint: the returned reference will be valid
// as long as both the parameters are valid. This is the relationship between lifetimes of the
// parameters and the return value.
//
// The function signature now tells Rust that for some lifetime 'a, the function takes two
// parameters, both of which are string slices that live at least as long as lifetime 'a. The
// function signature also tells Rust that the string slice returned from the function will live at
// least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned
// by the longest function is the same as the smaller of the lifetimes of the values referred to by
// the function arguments. These relationships are what we want Rust to use when analyzing this
// code.
//
// The lifetime annotation tells the borrow checker to reject any values where the above is not
// true. It's essentially an extension of the contract of the function like the type parameters.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn longester(s1: String, s2: String) -> String {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}
