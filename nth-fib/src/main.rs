fn main() {
    println!("Input the index of the fibonacci number you'd like to calculate");

    let mut n = String::new();

    std::io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Failed to convert input to number");

    println!("{n}. fibonacci number is: {}", fib(n))
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        n + fib(n - 1)
    }
}
