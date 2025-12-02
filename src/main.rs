use std::{env::args, time::Instant};

use crate::{day1::Day1, day2::Day2};

pub mod day1;
pub mod day2;

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

trait Day {
    fn solve(&self);
}

struct Solver {
    day: Box<dyn Day>,
}

impl Solver {
    fn new(day: impl Day + 'static) -> Self {
        Self { day: Box::new(day) }
    }

    fn solve(&self) {
        self.day.solve();
    }
}

fn main() {
    let days = vec![Solver::new(Day1 {}), Solver::new(Day2 {})];
    args()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|i| usize::from_str_radix(i, 10).unwrap())
        .for_each(|day| {
            println!("Day {day}");
            days[day - 1].solve();
            println!("===");
        });
}
