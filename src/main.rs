use std::{env::args, time::Instant};

pub mod day1;
pub mod day2;

trait Puzzle {
    fn solve(&self);
}

struct Timer {
    start: Instant,
}

impl Timer {
    fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let end = Instant::now();
        println!("Took {}s", end.duration_since(self.start).as_secs_f64())
    }
}

fn main() {
    let days: Vec<Box<dyn Puzzle>> = vec![Box::new(day1::Day1 {}), Box::new(day2::Day2 {})];
    let day = usize::from_str_radix(&args().nth(1).unwrap(), 10).unwrap();
    days[day - 1].solve();
}
