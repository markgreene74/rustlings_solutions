// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.

fn main() {
    // see https://doc.rust-lang.org/rust-by-example/primitives/tuples.html
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
