use crate::{Day, Timer};

pub struct Day2;

impl Day for Day2 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day2.txt");

        {
            let _timer = Timer::new();
            let sum = input
                .split(',')
                .map(|pair| {
                    pair.split('-').nth(0).unwrap().parse::<i64>().unwrap()
                        ..=pair.split('-').nth(1).unwrap().parse::<i64>().unwrap()
                })
                .map(|pair| {
                    pair.filter(|id| {
                        let id_string = id.to_string();
                        let len = id_string.len();
                        let (a, b) = (&id_string[..len / 2], &id_string[len / 2..]);
                        a == b
                    })
                    .sum::<i64>()
                })
                .sum::<i64>();
            println!("Part 1: {sum}");
        }

        {
            let _timer = Timer::new();
            let sum = input
                .split(',')
                .map(|pair| {
                    pair.split('-').nth(0).unwrap().parse::<i64>().unwrap()
                        ..=pair.split('-').nth(1).unwrap().parse::<i64>().unwrap()
                })
                .map(|pair| {
                    pair.filter(|id| {
                        let id_string = id.to_string();
                        let len = id_string.len();
                        (1..=len / 2).any(|i| {
                            let seq = &id_string.as_bytes()[..i];
                            id_string
                                .as_bytes()
                                .chunks(i)
                                .skip(1)
                                .all(|chunk| chunk == seq)
                        })
                    })
                    .sum::<i64>()
                })
                .sum::<i64>();
            println!("Part 2: {sum}");
        }
    }
}
