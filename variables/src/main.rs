fn main() {
    let mut x = 5;
    static A: &str = "a";
    println!("{A}");
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let quotient = 56.7 / 32.2;
    println!("The quotient: {quotient}");
}
