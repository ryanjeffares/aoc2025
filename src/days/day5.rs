use crate::{Day, Timer};

pub struct Day5;

impl Day for Day5 {
    fn solve(&self) {
        let input_ranges = include_str!("../../inputs/day5_ranges.txt");
        let mut ranges = input_ranges
            .split('\n')
            .map(|line| {
                (
                    line.trim()
                        .split('-')
                        .nth(0)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                    line.trim()
                        .split('-')
                        .nth(1)
                        .unwrap()
                        .parse::<u64>()
                        .unwrap(),
                )
            })
            .collect::<Vec<(u64, u64)>>();
        ranges.sort_by(|a, b| a.0.cmp(&b.0));

        {
            let _timer = Timer::new();
            let ids = include_str!("../../inputs/day5_ids.txt")
                .split('\n')
                .map(|line| line.trim().parse::<u64>().unwrap());
            let num_fresh_ids = ids
                .filter(|id| {
                    for (a, b) in ranges.iter() {
                        if a > id {
                            return false;
                        }

                        if id >= a && id <= b {
                            return true;
                        }
                    }

                    false
                })
                .count();

            println!("Part 1: {num_fresh_ids}");
        }

        {
            let _timer = Timer::new();
            let mut cleaned_ranges: Vec<(u64, u64)> = Vec::<(u64, u64)>::new();
            let (mut start, mut end) = ranges[0].clone();
            for (a, b) in ranges.into_iter().skip(1) {
                if a > start && a > end {
                    cleaned_ranges.push((start, end));
                    (start, end) = (a, b);
                } else if b > end {
                    end = b;
                }
            }
            cleaned_ranges.push((start, end));
            let fresh_ids = cleaned_ranges.iter().map(|(a, b)| b - a + 1).sum::<u64>();

            println!("Part 2: {fresh_ids}");
        }
    }
}
