use std::io;

fn read_i32(prompt: &str) -> i32 {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Not a valid number")
}
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn digit_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn count_divisors(n: i32) -> i32 {
    let mut count = 0;
    for i in 1..=n {
        if n % i == 0 {
            count += 1;
        }
    }
    count
}
fn main() {
    let start = read_i32("Start of range:");
let end = read_i32("End of range:");
for n in start..=end {
    println!(
        "{n}: even={}, digit_sum={}, prime={}, divisors={}",
        is_even(n), digit_sum(n), is_prime(n), count_divisors(n)
    );
}          
}
