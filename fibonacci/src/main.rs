use std::io;

fn main() {
    let mut n = String::new();
    println!("Please enter the number of the fiaonacci sequence you wish to know");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line!");

    let n = n.trim().parse::<u32>().expect("failed to parse");
    println!("the {}th number in fibaonacci seq is: {}", n, fibonacci(n))
}

fn fibonacci(n: u32) -> u32 {
    let mut fib: Vec<u32> = vec![0, 1];
    if n < (2_u32) {
        return fib[n as usize];
    }
    for i in 2..=n as usize {
        fib.push(fib[i - 1] + fib[i - 2]);
    }

    println!("fib: {:?}", fib);
    fib[n as usize]
}
