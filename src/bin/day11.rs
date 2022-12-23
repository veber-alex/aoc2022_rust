use std::{collections::VecDeque, convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: fn(usize, usize) -> usize,
    operation_val: usize,
    test_div_by: usize,
    test_if_true: usize,
    test_if_false: usize,
    inspected: usize,
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();

        let starting_items = lines.next().unwrap();
        let items: VecDeque<_> = starting_items
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let mut operation = lines.next().unwrap().rsplitn(3, ' ');
        let mul = operation.next().unwrap();
        let op = operation.next().unwrap();
        let operation = match mul {
            "old" => |old, _| old * old,
            _ => match op {
                "*" => |old, val| old * val,
                "+" => |old, val| old + val,
                _ => unreachable!(),
            },
        };
        let operation_mul = mul.parse().unwrap_or(0);

        let mut parse = || {
            lines
                .next()
                .unwrap()
                .rsplit(' ')
                .next()
                .unwrap()
                .trim()
                .parse()
                .unwrap()
        };

        Ok(Monkey {
            items,
            operation,
            operation_val: operation_mul,
            test_div_by: parse(),
            test_if_true: parse(),
            test_if_false: parse(),
            inspected: 0,
        })
    }
}

fn main() {
    let input = include_str!("../input/day11.txt");
    let monkeys_start: Vec<Monkey> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();

    // part 1
    let mut monkeys = monkeys_start.clone();
    let part1 = throw_stuff(&mut monkeys, 20, |worry| worry / 3);
    assert_eq!(part1, 98280);

    // part 2
    let mut monkeys = monkeys_start;
    let modulo: usize = monkeys.iter().map(|m| m.test_div_by).product();
    let part2 = throw_stuff(&mut monkeys, 10000, |worry| worry % modulo);
    assert_eq!(part2, 17673687232);
}

fn throw_stuff(
    monkeys: &mut Vec<Monkey>,
    rounds: usize,
    mut worry_reducer: impl FnMut(usize) -> usize,
) -> usize {
    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                let monkey = &mut monkeys[idx];
                monkey.inspected += 1;
                let worry = worry_reducer((monkey.operation)(item, monkey.operation_val));
                let new_idx = if worry % monkey.test_div_by == 0 {
                    monkey.test_if_true
                } else {
                    monkey.test_if_false
                };
                monkeys[new_idx].items.push_back(worry);
            }
        }
    }

    monkeys.sort_unstable_by_key(|monkey| monkey.inspected);
    monkeys[monkeys.len() - 1].inspected * monkeys[monkeys.len() - 2].inspected
}
