use crate::{Day, Timer};

pub struct Day1;

impl Day for Day1 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day1.txt");

        {
            let _timer = Timer::new();
            let num_zeroes = input
                .split('\n')
                .map(|line| line.trim())
                .scan(50, |dial, line| {
                    let dir = line.as_bytes()[0];
                    let rot = line[1..].parse::<i32>().unwrap() % 100;
                    if dir == b'R' {
                        *dial = (*dial + rot) % 100;
                    } else {
                        *dial = *dial - rot;
                        if *dial < 0 {
                            *dial += 100;
                        }
                    }
                    Some(*dial)
                })
                .filter(|&num| num == 0)
                .count();

            println!("Part 1: {num_zeroes}");
        }

        {
            let _timer = Timer::new();
            let num_zeroes = input
                .split('\n')
                .map(|line| line.trim())
                .scan(50, |dial, line| {
                    let dir = line.as_bytes()[0];
                    let rot: i32 = line[1..].parse().unwrap();
                    let num_zeroes = rot / 100;
                    let rot = rot % 100;
                    let is_zero = *dial == 0;

                    if dir == b'R' {
                        *dial += rot;
                        match *dial {
                            d if d > 100 => {
                                *dial -= 100;
                                Some(num_zeroes + 1)
                            }
                            d if d == 100 => {
                                *dial = 0;
                                Some(num_zeroes + 1)
                            }
                            d if d == 0 => Some(num_zeroes + 1),
                            _ => Some(num_zeroes),
                        }
                    } else {
                        *dial -= rot;
                        match *dial {
                            d if d < 0 => {
                                *dial += 100;
                                if is_zero {
                                    Some(num_zeroes)
                                } else {
                                    Some(num_zeroes + 1)
                                }
                            }
                            d if d == 0 => Some(num_zeroes + 1),
                            _ => Some(num_zeroes),
                        }
                    }
                })
                .sum::<i32>();

            println!("Part 2: {num_zeroes}");
        }
    }
}
