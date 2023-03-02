use loopover::error::ErrorMessage;

mod loopover;

fn main() {
    let result = loopover::solve(std::env::args());
    println!("{}", result);
}
