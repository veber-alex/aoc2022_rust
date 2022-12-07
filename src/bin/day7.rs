fn main() {
    let input = include_str!("../input/day7.txt");

    let mut growing_dirs: Vec<usize> = vec![];
    let mut done_dirs: Vec<usize> = vec![];
    for s in input.lines().filter_map(|line| {
        let (prefix, rest) = line.split_once(' ').unwrap();
        match prefix {
            "$" => rest.split_once(' ').map(|(_, b)| b),
            "dir" => None,
            _ => Some(prefix),
        }
    }) {
        if let Ok(size) = s.parse::<usize>() {
            growing_dirs.iter_mut().for_each(|s| *s += size);
        } else if s == ".." {
            done_dirs.push(growing_dirs.pop().unwrap())
        } else {
            growing_dirs.push(0)
        }
    }

    // part 1
    let part1 = done_dirs
        .iter()
        .chain(&growing_dirs)
        .copied()
        .filter(|&s| s <= 100000)
        .sum::<usize>();
    assert_eq!(part1, 1334506);

    // part 2
    let total_size = growing_dirs[0];
    let required_size = 30000000 - (70000000 - total_size);
    let part2 = done_dirs
        .iter()
        .chain(&growing_dirs)
        .copied()
        .filter(|&s| s >= required_size)
        .min()
        .unwrap();
    assert_eq!(part2, 7421137);
}
