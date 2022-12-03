fn main() {
    let input = include_str!("../input/day3.txt");

    // part 1
    let mut table = [false; 53];
    let mut prio_sum = 0;

    for line in input.lines() {
        let half = line.len() / 2;
        for &item in line[..half].as_bytes() {
            table[item_to_prio(item)] = true;
        }
        for &item in line[half..].as_bytes() {
            let prio = item_to_prio(item);
            if table[prio] {
                prio_sum += prio;
                table.fill(false);
                break;
            }
        }
    }

    assert_eq!(prio_sum, 7875);

    // part 2
    let mut table: [u8; 53] = [0; 53];
    let mut prio_sum = 0;
    let mut lines = input.lines();

    while let Some(line1) = lines.next() {
        for &item in line1.as_bytes() {
            table[item_to_prio(item)] = 1;
        }
        for &item in lines.next().unwrap().as_bytes() {
            let slot = &mut table[item_to_prio(item)];
            if *slot == 1 {
                *slot = 2;
            }
        }
        for &item in lines.next().unwrap().as_bytes() {
            let prio = item_to_prio(item);
            if table[prio] == 2 {
                prio_sum += prio;
                table.fill(0);
                break;
            }
        }
    }

    assert_eq!(prio_sum, 2479);
}

fn item_to_prio(mut item: u8) -> usize {
    if let b'a'..=b'z' = item {
        item -= 58
    };
    (item - 38) as usize
}
