use crate::{Puzzle, Timer};

pub struct Day1;

impl Puzzle for Day1 {
    fn solve(&self) {
        let input = include_str!("../inputs/day1.txt");

        {
            let _timer = Timer::new();
            let mut dial = 50;

            let num_zeroes = input
                .split('\n')
                .filter(|line| {
                    let line = line.trim();
                    let dir = line.as_bytes()[0];
                    let rot = i32::from_str_radix(&line[1..], 10).unwrap() % 100;

                    if dir == b'R' {
                        dial = (dial + rot) % 100
                    } else {
                        dial -= rot;

                        if dial < 0 {
                            dial = 100 + dial;
                        }
                    }

                    dial == 0
                })
                .count();

            println!("{num_zeroes}");
        }

        {
            let _timer = Timer::new();
            let mut num_zeroes = 0;
            let mut dial = 50;

            for line in input.split('\n') {
                let line = line.trim();
                let dir = line.as_bytes()[0];
                let rot = i32::from_str_radix(&line[1..], 10).unwrap();
                num_zeroes += rot / 100;
                let rot = rot % 100;
                let is_zero = dial == 0;

                if dir == b'R' {
                    dial += rot;

                    if dial > 100 {
                        if !is_zero {
                            num_zeroes += 1;
                        }

                        dial -= 100;
                    } else if dial == 100 {
                        dial = 0;
                    }
                } else {
                    dial -= rot;

                    if dial < 0 {
                        if !is_zero {
                            num_zeroes += 1;
                        }

                        dial = 100 + dial;
                    }
                }

                if dial == 0 {
                    num_zeroes += 1;
                }
            }

            println!("{num_zeroes}");
        }
    }
}
