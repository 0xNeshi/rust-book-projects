type TR = (String, String);
type TR2 = (String, String);

// struct STR(String, String);
// struct STR2(String, String);

fn main() {
    let tr: TR = (String::from("a"), String::from("a"));
    let tr2: TR2 = (String::from("a"), String::from("a"));
    func(tr2);
}

fn func(t: TR) {}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
