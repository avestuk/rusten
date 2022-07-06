use std::io;

fn main() {
    let mut n = String::new();
    println!("Please enter the number of the fiaonacci sequence you wish to know");
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read the line!");

    let n = n.trim().parse::<usize>().expect("failed to parse");
    println!("the {}th number in fibaonacci seq is: {}", n, fibonacci(n))
}

fn fibonacci(n: usize) -> usize {
    let mut fib: Vec<usize> = vec![0, 1];
    if n < (2_usize) {
        return fib[n as usize];
    }
    for i in 2..=n as usize {
        fib.push(fib[i - 1] + fib[i - 2]);
    }

    println!("fib: {:?}", &fib[1..]);
    fib[n]
}
