/// A "Hello, world!" program.
///
/// This is the best implementation of this program to ever exist.
/// The main function runs when our program starts
fn main() {
    print(hello());
}

fn print(s: String) -> () {
    println!("{s}");
}

fn hello() -> String {
    return "Hello, world!".into();
}
