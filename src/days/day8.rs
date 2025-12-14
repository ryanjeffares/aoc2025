use crate::{Day, Timer};

pub struct Day8;

type Point = (u64, u64, u64);

fn distance(a: &Point, b: &Point) -> u64 {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

fn find(mut x: usize, parent: &mut Vec<usize>) -> usize {
    let mut root = x;
    while parent[root] != root {
        root = parent[root];
    }

    while parent[x] != root {
        let p = parent[x];
        parent[x] = root;
        x = p;
    }

    root
}

fn union(mut x: usize, mut y: usize, parent: &mut Vec<usize>, size: &mut Vec<usize>) -> bool {
    x = find(x, parent);
    y = find(y, parent);

    if x == y {
        return false;
    }

    if size[x] < size[y] {
        (x, y) = (y, x);
    }

    parent[y] = x;
    size[x] += size[y];
    size[y] = 0;
    true
}

impl Day for Day8 {
    fn solve(&self) {
        let input = include_str!("../../inputs/day8.txt");
        let points = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut split = line.trim().split(',');
                (
                    split.next().unwrap().parse::<u64>().unwrap(),
                    split.next().unwrap().parse::<u64>().unwrap(),
                    split.next().unwrap().parse::<u64>().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let mut input_pairs = (0..points.len())
            .map(|i| (i + 1..points.len()).map(move |j| (i, j)))
            .flatten()
            .collect::<Vec<_>>();
        input_pairs.sort_by_key(|(i, j)| distance(&points[*i], &points[*j]));
        let input_pairs = input_pairs;

        {
            let _timer = Timer::new();

            let mut parent = (0..points.len()).collect::<Vec<_>>();
            let mut size = vec![1; points.len()];

            for &(i, j) in input_pairs.iter().take(1000) {
                union(i, j, &mut parent, &mut size);
            }

            let mut sorted = size.into_iter().filter(|&x| x != 0).collect::<Vec<_>>();
            sorted.sort();
            let product = sorted.iter().rev().take(3).product::<usize>();

            println!("Part 1: {product}");
        }

        {
            let _timer = Timer::new();

            let mut parent = (0..points.len()).collect::<Vec<_>>();
            let mut size = vec![1; points.len()];
            let mut num_circuits_removed = 0;
            let mut product = None;

            for &(i, j) in &input_pairs {
                if union(i, j, &mut parent, &mut size) {
                    num_circuits_removed += 1;
                    if num_circuits_removed == size.len() - 1 {
                        product = Some(points[i].0 * points[j].0);
                        break;
                    }
                }
            }

            println!("Part 1: {}", product.unwrap());
        }
    }
}
