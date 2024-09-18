fn main() {
    println!("Input temperature in Fahrenheit!");

    let mut fahrenheit = String::new();

    std::io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit: u32 = fahrenheit
        .trim()
        .parse()
        .expect("Failed to convert string to fahrenheit");

    let celsius = (fahrenheit - 32) * 5 / 9;

    println!("Converted to celsius: {celsius}");
}
