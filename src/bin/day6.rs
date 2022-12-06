fn main() {
    let input = include_bytes!("../input/day6.txt");

    // part 1
    assert_eq!(solve(input, 4), 1855);

    // part 2
    assert_eq!(solve(input, 14), 3256);
}

fn solve(input: &[u8], window: usize) -> usize {
    let mut result = 0;
    'out: for (idx, slice) in input.windows(window).enumerate() {
        let mut table = [false; 26];
        for &b in slice {
            let b_pos = (b - 97) as usize;
            if table[b_pos] {
                continue 'out;
            }
            table[b_pos] = true;
        }
        result = idx + window;
        break;
    }
    result
}
