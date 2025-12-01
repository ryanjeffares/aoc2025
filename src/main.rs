use std::env::args;

trait Puzzle {
    fn solve(&self);
}

pub mod day1;

fn main() {
    let days: Vec<Box<dyn Puzzle>> = vec![Box::new(day1::Day1 {})];
    let day = usize::from_str_radix(&args().nth(1).unwrap(), 10).unwrap();
    days[day - 1].solve();
}
