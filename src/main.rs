/// A "Hello, world!" program.
///
/// This is the best implementation of this program to ever exist.

fn main() {
    print(hello());
}

fn print(s: String) -> () {
    println!("{s}");
}

fn hello() -> String {
    return "Hello, world!".into();
}
