use std::collections::HashMap;

fn main() {
    // vectors
    let v = vec![1, 2, 3];

    let third: &i32 = &v[2];
    println!("The third element is: {third}");

    // get provides an Option so you can check if the element exists or not
    // [] panics if an element is out of bounds.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There was no third element"),
    }

    // We can range over elements in the vector
    for i in &v {
        println!("element: {i}")
    }

    // Or we can do the ranging with an iterator
    let even_total: i32 = v.iter().filter(|x| x.is_positive()).sum();
    println!("The total of even elements is: {even_total}");

    // Strings
    let mut s = String::new();
    let data = "initial contents";

    s = data.to_string();
    println!("s: {s}");

    // We can iterate over strings
    for c in s.chars() {
        println!("char: {c}")
    }

    // Hash Maps
    let mut scores = HashMap::new();

    // The type is inferred after the first addition
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 5);

    let team_name = String::from("blue");
    let blue_score = scores.get(&team_name).unwrap_or(&0);

    // You can do conditional insertion with .entry
    // If the key exists the value is returned, otherwise a value is inserted
    scores.entry(String::from("red")).or_insert(80);
    scores.entry(String::from("green")).or_insert(80);
    println!("{:?}", scores);
}
