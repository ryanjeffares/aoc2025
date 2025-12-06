use std::{env::args, time::Instant};

use days::day6::Day6;

use crate::days::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, day5::Day5};

pub mod days;

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
    let days = vec![
        Solver::new(Day1 {}),
        Solver::new(Day2 {}),
        Solver::new(Day3 {}),
        Solver::new(Day4 {}),
        Solver::new(Day5 {}),
        Solver::new(Day6 {}),
    ];
    args()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|i| i.parse::<usize>().unwrap())
        .for_each(|day| {
            println!("Day {day}");
            days[day - 1].solve();
            println!("===");
        });
}
