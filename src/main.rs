use loopover::error::ErrorMessage;

mod loopover;

fn main() {
    loopover::solve(std::env::args());
}
