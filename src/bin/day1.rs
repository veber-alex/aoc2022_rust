fn main() {
    let input = include_str!("../input/day1.txt");

    // part 1
    let mut calories = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();
    calories.sort_unstable();

    let len = calories.len();

    let max = calories[len - 1];
    assert_eq!(max, 66487);

    // part 2
    let max_1 = calories[len - 2];
    let max_2 = calories[len - 3];
    assert_eq!(max + max_1 + max_2, 197301);
}
