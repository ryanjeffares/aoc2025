use std::cmp::Ordering;

use crate::{Day, Timer};

pub struct Day3;

impl Day for Day3 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day3.txt");

        {
            let _timer = Timer::new();
            let total_joltage = input
                .split('\n')
                .map(|line| find_largest_joltage(line.trim().as_bytes(), 2))
                .sum::<usize>();

            println!("Part 1: {total_joltage}");
        }

        {
            let _timer = Timer::new();
            let total_joltage = input
                .split('\n')
                .map(|line| find_largest_joltage(line.trim().as_bytes(), 12))
                .sum::<usize>();

            println!("Part 2: {total_joltage}");
        }
    }
}

fn find_largest_joltage(nums: &[u8], number_length: usize) -> usize {
    (0..number_length)
        .fold((usize::MAX, 0), |(last_index, acc), i| {
            let i = number_length - i;
            let start_index = last_index.wrapping_add(1);
            let end_index = nums.len() - i;
            let (max_index, &max) = nums[start_index..=end_index]
                .iter()
                .enumerate()
                .max_by(|(_, val1), (_, val2)| {
                    let ord = val1.cmp(val2);
                    match ord {
                        Ordering::Equal => Ordering::Less,
                        _ => ord,
                    }
                })
                .unwrap();
            (
                max_index + start_index,
                acc + (usize::from(max) - 48) * 10usize.pow(i as u32 - 1),
            )
        })
        .1
}
