use crate::{Day, Timer};

pub struct Day9;

impl Day for Day9 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day9.txt");

        {
            let _timer = Timer::new();
            let points = input
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let mut split = line.split(',');
                    (
                        split.next().unwrap().parse::<i64>().unwrap(),
                        split.next().unwrap().parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            let max_area = (0..points.len())
                .map(|i| (i + 1..points.len()).map(move |j| (i, j)))
                .flatten()
                .map(|(i, j)| {
                    let (a, b) = (points[i], points[j]);
                    let x = (a.0 - b.0).abs() + 1;
                    let y = (a.1 - b.1).abs() + 1;
                    x * y
                })
                .max()
                .unwrap();

            println!("Part 1: {max_area}");
        }
    }
}
