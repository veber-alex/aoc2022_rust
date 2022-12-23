use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct TH {
    x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("../input/day9.txt");
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (inst, n) = line.split_once(' ').unwrap();
            (inst, n.parse::<u32>().unwrap())
        })
        .collect();

    let mut th = [TH::default(); 10];
    let mut moveset_part1 = HashSet::from([th[1]]);
    let mut moveset_part2 = HashSet::from([th[9]]);

    for &(inst, n) in &instructions {
        let move_head: fn(&mut TH) = match inst {
            "R" => |th| th.x += 1,
            "L" => |th| th.x -= 1,
            "U" => |th| th.y += 1,
            "D" => |th| th.y -= 1,
            _ => unreachable!(),
        };
        for _ in 0..n {
            move_head(&mut th[0]);
            for i in 0..9 {
                move_tail(th[i], &mut th[i + 1]);
            }
            moveset_part1.insert(th[1]);
            moveset_part2.insert(th[9]);
        }
    }

    // part 1
    let part1 = moveset_part1.len();
    assert_eq!(part1, 6503);

    // part 2
    let part2 = moveset_part2.len();
    assert_eq!(part2, 2724);
}

fn move_tail(head: TH, mut tail: &mut TH) {
    let diff_x = head.x - tail.x;
    let diff_y = head.y - tail.y;

    if !matches!(diff_x, -1..=1) || !matches!(diff_y, -1..=1) {
        tail.x += diff_x.clamp(-1, 1);
        tail.y += diff_y.clamp(-1, 1);
    }
}
