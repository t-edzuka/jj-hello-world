/// A "Hello, world!" program.
///
/// This is the best implementation of this program to ever exist.
/// The main function runs when our program starts
fn main() {
    print(hello());
    print(goodbye());
}

fn print(s: String) -> () {
    println!("{s}");
}

fn hello() -> String {
    "Hello, world!".into()
}

fn goodbye() -> String {
    "Good bye, world!".into()
}
