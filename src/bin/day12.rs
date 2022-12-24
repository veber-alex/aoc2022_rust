#![allow(clippy::unnecessary_lazy_evaluations)]

use std::{cmp::Reverse, collections::BinaryHeap};

struct Vertex {
    id: usize,
    priority: Reverse<u32>,
}

impl Vertex {
    fn new(id: usize, priority: u32) -> Self {
        Self {
            id,
            priority: Reverse(priority),
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

fn shortest_path(data: &[u8], stride: usize, start: usize, target: usize) -> Option<usize> {
    let mut q = BinaryHeap::with_capacity(data.len());
    let mut dist = vec![usize::MAX; data.len()];

    let check_height = |from: usize, to: usize| -> bool {
        let from = data[from];
        let to = data[to];

        if to > from {
            to - from == 1
        } else {
            true
        }
    };

    dist[start] = 0;

    q.push(Vertex::new(start, 0));

    while let Some(Vertex { id: u, .. }) = q.pop() {
        if u == target {
            return Some(dist[target]);
        }

        [
            // down
            (u + stride < data.len() && check_height(u, u + stride)).then(|| u + stride),
            // right
            ((u + 1) % stride != 0 && check_height(u, u + 1)).then(|| u + 1),
            // up
            u.checked_sub(stride)
                .and_then(|nu| check_height(u, nu).then_some(nu)),
            // left
            (u % stride != 0 && check_height(u, u - 1)).then(|| u - 1),
        ]
        .into_iter()
        .flatten()
        .for_each(|v| {
            let alt = dist[u] + 1;
            if alt < dist[v] {
                dist[v] = alt;
                q.push(Vertex::new(v, alt as u32));
            }
        });
    }

    None
}

fn main() {
    let input = include_str!("../input/day12.txt");

    let mut data: Vec<u8> = input
        .lines()
        .flat_map(|line| line.as_bytes().iter().copied())
        .collect();
    let stride = input.lines().next().unwrap().len();

    // part 1
    let start = data.iter().position(|&b| b == b'S').unwrap();
    data[start] = b'a';
    let target = data.iter().position(|&b| b == b'E').unwrap();
    data[target] = b'z';
    let part1 = shortest_path(&data, stride, start, target).unwrap();
    assert_eq!(part1, 462);

    // part 1
    let part2 = data
        .iter()
        .enumerate()
        .filter(|&(_, x)| *x == b'a')
        .flat_map(|(start, _)| shortest_path(&data, stride, start, target))
        .min()
        .unwrap();
    assert_eq!(part2, 451);
}
