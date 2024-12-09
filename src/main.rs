/// A "Hello, world!" program.
///
/// This is the best implementation of this program to ever exist.

fn main() {
    print(hello());
    print(goodbye());
}

fn print(s: String) -> () {
    println!("{s}");
}

fn hello() -> String {
    return "Hello, world!".into();
}

fn goodbye() -> String {
    return "Goodbye, world!".into();
}
