fn main() {
    let input = include_str!("../input/day4.txt");

    let mut contained = 0;
    let mut overlap = 0;
    for line in input.lines() {
        let ((lo1, hi1), (lo2, hi2)) = line
            .split_once(',')
            .map(|(r1, r2)| {
                fn split_and_parse(r: &str) -> (u32, u32) {
                    r.split_once('-')
                        .map(|(x1, x2)| (x1.parse::<u32>().unwrap(), x2.parse::<u32>().unwrap()))
                        .unwrap()
                }
                (split_and_parse(r1), split_and_parse(r2))
            })
            .unwrap();

        // part 1
        if (lo1 <= lo2 && hi1 >= hi2) || (lo2 <= lo1 && hi2 >= hi1) {
            contained += 1;
        }

        // part 2
        if !((hi1 < lo2) || (hi2 < lo1)) {
            overlap += 1;
        }
    }

    assert_eq!(contained, 507);
    assert_eq!(overlap, 897);
}
