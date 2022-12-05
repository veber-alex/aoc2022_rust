fn main() {
    let input = include_str!("../input/day5.txt");

    let (stacks_txt, procedure) = input.split_once("\n\n").unwrap();

    let mut lines = stacks_txt.lines().rev();
    let numbers_line = lines.next().unwrap().trim();
    let num_stacks = numbers_line[numbers_line.len() - 1..]
        .parse::<usize>()
        .unwrap();
    let mut stacks_part1 = vec![vec![]; num_stacks];

    for line in lines {
        for (idx, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks_part1[idx].push(c);
            }
        }
    }
    let mut stacks_part2 = stacks_part1.clone();

    for line in procedure.lines() {
        let mut iter = line.split_ascii_whitespace().skip(1).step_by(2);
        let num = iter.next().unwrap().parse::<usize>().unwrap();
        let from = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let to = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        let mut buf = vec![];
        for _ in 0..num {
            // part 1
            let item = stacks_part1[from].pop().unwrap();
            stacks_part1[to].push(item);

            // part 2
            let item = stacks_part2[from].pop().unwrap();
            buf.push(item);
        }
        stacks_part2[to].extend(buf.drain(..).rev());
    }

    let part1 = stacks_part1
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    let part2 = stacks_part2
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>();

    assert_eq!(part1, "WSFTMRHPP");
    assert_eq!(part2, "GSLCMFBRP");
}
