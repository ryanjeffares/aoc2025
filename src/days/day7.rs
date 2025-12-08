use std::collections::{HashMap, HashSet};

use crate::{Day, Timer};

pub struct Day7;

impl Day for Day7 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day7.txt")
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.as_bytes())
            .collect::<Vec<&[u8]>>();

        {
            let _timer = Timer::new();
            let num_splits = input
                .iter()
                .skip(2)
                .step_by(2)
                .scan(HashSet::from([input[0].len() / 2]), |beams, line| {
                    Some(
                        line.iter()
                            .enumerate()
                            .filter(|(i, c)| {
                                **c == b'^' && {
                                    if beams.contains(i) {
                                        beams.remove(i);
                                        beams.insert(i + 1);
                                        beams.insert(i - 1);
                                        true
                                    } else {
                                        false
                                    }
                                }
                            })
                            .count(),
                    )
                })
                .sum::<usize>();

            println!("Part 1: {num_splits}");
        }

        {
            let _timer = Timer::new();
            let beam = input[0].len() / 2;
            let row = 0;
            let mut visited = HashMap::<(usize, usize), usize>::new();
            let num_paths = dfs(beam, row, &input, &mut visited);
            println!("Part 2: {num_paths}");
        }
    }
}

fn dfs(
    x: usize,
    y: usize,
    graph: &Vec<&[u8]>,
    visited: &mut HashMap<(usize, usize), usize>,
) -> usize {
    for y in (y + 2..graph.len()).step_by(2) {
        if graph[y][x] == b'^' {
            if let Some(num_paths) = visited.get(&(x, y)) {
                return *num_paths;
            } else {
                let num_paths = dfs(x + 1, y, graph, visited) + dfs(x - 1, y, graph, visited);
                visited.insert((x, y), num_paths);
                return num_paths;
            }
        }
    }

    1
}
