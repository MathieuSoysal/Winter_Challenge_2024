pub mod fib;
use fib::fibonacci;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n = args
        .get(1)
        .expect("Give one argument")
        .parse::<u32>()
        .expect("Given argument should be a integer.");

    println!("{}", fibonacci(n));
}
