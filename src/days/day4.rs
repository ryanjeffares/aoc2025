use crate::{Day, Timer};

pub struct Day4;

impl Day for Day4 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day4.txt");
        let grid: Vec<Vec<u8>> = input
            .split('\n')
            .map(|line| line.trim().as_bytes().iter().copied().collect())
            .collect();
        let offsets = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        {
            let _timer = Timer::new();
            let num_accessible_rolls = grid
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(x, roll)| {
                            **roll == b'@'
                                && offsets
                                    .iter()
                                    .filter(|(x_offset, y_offset)| {
                                        let x = (*x as i32) + x_offset;
                                        let y = (y as i32) + y_offset;
                                        (x >= 0
                                            && x < row.len() as i32
                                            && y >= 0
                                            && y < grid.len() as i32)
                                            && grid[y as usize][x as usize] == b'@'
                                    })
                                    .count()
                                    < 4
                        })
                        .count()
                })
                .sum::<usize>();

            println!("Part 1: {num_accessible_rolls}");
        }

        {
            let _timer = Timer::new();

            let rolls_removed = std::iter::repeat(())
                .scan(grid, |grid, _| {
                    let accessible_rolls: Vec<(usize, usize)> = grid
                        .iter()
                        .enumerate()
                        .map(|(y, row)| {
                            row.iter()
                                .enumerate()
                                .filter_map(|(x, roll)| {
                                    if *roll == b'@'
                                        && offsets
                                            .iter()
                                            .filter(|(x_offset, y_offset)| {
                                                let x = (x as i32) + x_offset;
                                                let y = (y as i32) + y_offset;
                                                (x >= 0
                                                    && x < row.len() as i32
                                                    && y >= 0
                                                    && y < grid.len() as i32)
                                                    && grid[y as usize][x as usize] == b'@'
                                            })
                                            .count()
                                            < 4
                                    {
                                        Some((x, y))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<(usize, usize)>>()
                        })
                        .flatten()
                        .collect();

                    for (x, y) in &accessible_rolls {
                        grid[*y][*x] = b'.';
                    }

                    if accessible_rolls.len() > 0 {
                        Some(accessible_rolls.len())
                    } else {
                        None
                    }
                })
                .reduce(|acc, x| acc + x)
                .unwrap();

            println!("Part 2: {rolls_removed}");
        }
    }
}
