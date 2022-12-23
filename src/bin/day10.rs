#[derive(Debug)]
enum Inst {
    NOP,
    AddX(i32),
}

impl Inst {
    fn from_str_risc(s: &str) -> [Option<Inst>; 2] {
        let second = match s {
            "noop" => None,
            _ => {
                let (_, n) = s.split_once(' ').unwrap();
                Some(Inst::AddX(n.parse().unwrap()))
            }
        };
        [Some(Inst::NOP), second]
    }
}

fn main() {
    let input = include_str!("../input/day10.txt");

    let instructions: Vec<Inst> = input
        .lines()
        .flat_map(|line| Inst::from_str_risc(line))
        .flatten()
        .collect();

    // part 1
    let mut check_cycle = 20;
    let mut x = 1;
    let mut signal = 0;

    for (inst, cycle) in instructions.iter().zip(1..) {
        if cycle == check_cycle {
            signal += cycle * x;
            check_cycle += 40;
        }
        if let Inst::AddX(addx) = inst {
            x += addx
        }
    }
    assert_eq!(signal, 12560);

    // part 2
    let mut x = 1;
    let mut part2 = String::new();

    for line in instructions.chunks_exact(40) {
        for (inst, cycle) in line.iter().zip(0..) {
            if (x - 1..=x + 1).contains(&cycle) {
                part2.push('#');
            } else {
                part2.push('.');
            }
            if let Inst::AddX(addx) = inst {
                x += addx
            }
        }
        part2.push('\n');
    }

    #[rustfmt::skip]
    let expected = 
"###..#....###...##..####.###...##..#....
#..#.#....#..#.#..#.#....#..#.#..#.#....
#..#.#....#..#.#..#.###..###..#....#....
###..#....###..####.#....#..#.#....#....
#....#....#....#..#.#....#..#.#..#.#....
#....####.#....#..#.#....###...##..####.
";
    assert_eq!(part2, expected);
}
