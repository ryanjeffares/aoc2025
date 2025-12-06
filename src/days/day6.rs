use crate::{Day, Timer};

pub struct Day6;

impl Day for Day6 {
    fn solve(&self) {
        let input_numbers = include_str!("../../inputs/day6_nums.txt");
        let input_ops = include_str!("../../inputs/day6_ops.txt");
        let ops = input_ops.split_whitespace().collect::<Vec<&str>>();

        {
            let _timer = Timer::new();
            let mut results: Vec<Option<u64>> = vec![None; ops.len()];
            input_numbers
                .split('\n')
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()
                })
                .for_each(|nums| {
                    for i in 0..nums.len() {
                        let op = ops[i];
                        if let Some(current) = results[i] {
                            match op {
                                "*" => results[i] = Some(current * nums[i]),
                                "+" => results[i] = Some(current + nums[i]),
                                _ => (),
                            }
                        } else {
                            results[i] = Some(nums[i]);
                        }
                    }
                });

            let total = results.iter().fold(0, |acc, n| acc + n.unwrap());
            println!("Part 1: {total}");
        }

        {
            let _timer = Timer::new();
            let char_map = input_numbers
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| line.as_bytes().iter().copied().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let row_len = char_map[0].len();
            let mut total = 0;
            let mut numbers = Vec::<u64>::new();
            let mut op_index = ops.len() - 1;

            for column in (0..row_len).rev() {
                let result = char_map
                    .iter()
                    .map(|row| row[column])
                    .filter(|&c| c != b' ')
                    .fold(0, |acc, c| {
                        if acc == 0 {
                            u64::from(c) - 48
                        } else {
                            acc * 10 + u64::from(c) - 48
                        }
                    });

                if result == 0 {
                    match ops[op_index] {
                        "*" => total += numbers.iter().product::<u64>(),
                        "+" => total += numbers.iter().sum::<u64>(),
                        _ => (),
                    }
                    if op_index > 0 {
                        op_index -= 1;
                    }
                    numbers.clear();
                } else {
                    numbers.push(result);
                }

                if column == 0 {
                    match ops[op_index] {
                        "*" => total += numbers.iter().product::<u64>(),
                        "+" => total += numbers.iter().sum::<u64>(),
                        _ => (),
                    }
                }
            }

            println!("Part 2: {total}");
        }
    }
}
