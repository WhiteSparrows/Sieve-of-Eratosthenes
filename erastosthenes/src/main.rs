use std::io;

use erastosthenes::naive;
use erastosthenes::optimize;

fn main() {
    println!("Please enter a number");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    let n: usize = n.trim().parse().unwrap();
    let prime_numbers = naive(n);
    let prime_numbers2 = optimize(n);
    println!("Prime numbers under {} are {:?}", n, prime_numbers);
    println!("Prime numbers under {} are {:?}", n, prime_numbers2);
}
