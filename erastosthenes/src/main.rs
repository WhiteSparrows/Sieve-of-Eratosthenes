use std::io;

fn main() {
    println!("Please enter a number");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");
    let n: i32 = n.trim().parse().unwrap();
    let mut prime_numbers = Vec::new();
    let mut vec: Vec<i32> = (2..n).collect();
    let mut p = 2;
    while !vec.is_empty(){
        vec.retain(|&x| x%p != 0);
        if !vec.is_empty(){
            prime_numbers.push(p);
            p = vec[0]
        }

    }
    println!("Prime numbers under {} are {:?}", n, prime_numbers)
}
