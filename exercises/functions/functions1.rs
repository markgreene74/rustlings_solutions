// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

fn main() {
    println!("main(), before call_me()");
    call_me();
    println!("main(), after call_me()");
}

fn call_me() {
    println!("executing call_me()");
}
