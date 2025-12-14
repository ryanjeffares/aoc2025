use std::collections::{HashMap, HashSet};

use crate::{Day, Timer};

pub struct Day9;

struct Point {
    x: i64,
    y: i64,
}

impl Day for Day9 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day9.txt");

        {
            let _timer = Timer::new();
            let points = input
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let mut split = line.trim().split(',');
                    Point {
                        x: split.next().unwrap().parse::<i64>().unwrap(),
                        y: split.next().unwrap().parse::<i64>().unwrap(),
                    }
                })
                .collect::<Vec<_>>();
            let max_area = (0..points.len())
                .map(|i| (i + 1..points.len()).map(move |j| (i, j)))
                .flatten()
                .map(|(i, j)| {
                    let (a, b) = (&points[i], &points[j]);
                    let x = (a.x - b.x).abs() + 1;
                    let y = (a.y - b.y).abs() + 1;
                    x * y
                })
                .max()
                .unwrap();

            println!("Part 1: {max_area}");
        }

        {
            let _timer = Timer::new();

            let points = input
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| {
                    let mut split = line.trim().split(',');
                    Point {
                        x: split.next().unwrap().parse::<i64>().unwrap(),
                        y: split.next().unwrap().parse::<i64>().unwrap(),
                    }
                })
                .collect::<Vec<_>>();

            let mut unique_x = points
                .iter()
                .map(|p| p.x)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            unique_x.sort();
            let mut x_map = HashMap::<i64, i64>::new();
            unique_x.iter().enumerate().for_each(|(i, &x)| {
                x_map.insert(x, i as i64);
            });

            let mut unique_y = points
                .iter()
                .map(|p| p.y)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            unique_y.sort();
            let mut y_map = HashMap::<i64, i64>::new();
            unique_y.iter().enumerate().for_each(|(i, &y)| {
                y_map.insert(y, i as i64);
            });

            let mut grid = vec![vec![b'.'; unique_x.len()]; unique_y.len()];

            let compressed_points = points
                .iter()
                .map(|p| {
                    let x = x_map[&p.x];
                    let y = y_map[&p.y];
                    grid[y as usize][x as usize] = b'#';
                    Point { x, y }
                })
                .collect::<Vec<_>>();

            for i in 0..compressed_points.len() {
                let (a, b) = (
                    &compressed_points[i],
                    &compressed_points[(i + 1) % compressed_points.len()],
                );

                if a.x == b.x {
                    let x = a.x as usize;
                    let (y_min, y_max) = (a.y.min(b.y) as usize, a.y.max(b.y) as usize);
                    for y in y_min..=y_max {
                        grid[y][x] = b'#';
                    }
                } else if a.y == b.y {
                    let y = a.y as usize;
                    let (x_min, x_max) = (a.x.min(b.x) as usize, a.x.max(b.x) as usize);
                    for x in x_min..=x_max {
                        grid[y][x] = b'#';
                    }
                }
            }

            fill(&mut grid);

            let max_area = (0..points.len())
                .map(|i| (i + 1..points.len()).map(move |j| (i, j)))
                .flatten()
                .filter_map(|(i, j)| {
                    let (a, b) = (&points[i], &points[j]);
                    let (x1, x2) = (x_map[&a.x], x_map[&b.x]);
                    let (y1, y2) = (y_map[&a.y], y_map[&b.y]);
                    let (x_min, x_max) = (x1.min(x2) as usize, x1.max(x2) as usize);
                    let (y_min, y_max) = (y1.min(y2) as usize, y1.max(y2) as usize);

                    for x in x_min..=x_max {
                        if grid[y_min][x] == b'.' || grid[y_max][x] == b'.' {
                            return None;
                        }
                    }

                    for y in y_min..=y_max {
                        if grid[y][x_min] == b'.' || grid[y][x_max] == b'.' {
                            return None;
                        }
                    }

                    let x = (a.x - b.x).abs() + 1;
                    let y = (a.y - b.y).abs() + 1;
                    Some(x * y)
                })
                .max()
                .unwrap();

            println!("Part 2: {max_area}");
        }
    }
}

fn get_inside_point(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != b'.' {
                continue;
            }

            let mut hits_left = 0;
            let mut prev = b'.';
            for i in (0..=x).rev() {
                let cur = grid[y][i];
                if cur != prev {
                    hits_left += 1;
                }
                prev = cur;
            }

            if hits_left % 2 == 1 {
                return (x, y);
            }
        }
    }

    unreachable!()
}

fn fill(grid: &mut Vec<Vec<u8>>) {
    let (x, y) = get_inside_point(grid);
    let mut stack = vec![(x, y)];
    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while stack.len() > 0 {
        let (x, y) = stack.pop().unwrap();
        if grid[y][x] != b'.' {
            continue;
        }
        grid[y][x] = b'X';

        for (x_offset, y_offset) in dirs {
            let x = (x as i64) + x_offset;
            let y = (y as i64) + y_offset;
            if y >= 0 && y < grid.len() as i64 && x >= 0 && x < grid[0].len() as i64 {
                let x = x as usize;
                let y = y as usize;
                if grid[y][x] == b'.' {
                    stack.push((x, y));
                }
            }
        }
    }
}
