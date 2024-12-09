/// A "Hello, world!" program.
///
/// This is the best implementation of this program to ever exist.

fn main() {
    println!("{}", hello())
    println!("{}", goodbye())
}

fn hello() -> String {
    return "Hello, world!".into()
}

fn goodbye() -> String {
    return "Goodbye, world!".into()
}
