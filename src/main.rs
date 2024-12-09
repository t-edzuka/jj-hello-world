/// A "Hello, world!" program.
///
/// This is the best implementation of this program to ever exist.
/// The main function runs when our program starts
fn main() {
    print(hello());
    print(goodbye());
    let two = (add(1, 1));
    let two_string = two.to_string();
    print(two_string)
}

fn print(s: String) {
    println!("{s}");
}

fn hello() -> String {
    "Hello, world!".into()
}

fn goodbye() -> String {
    "Good bye, world!".into()
}

fn add(a: u8, b: u8) -> u8 {
    a + b
}
