// answer is 1676

fn solve(items: &Vec<u16>) -> u16 {
    let mut prev: u16 = 0;
    let mut count: u16 = 0;
    for item in items.into_iter() {
        if prev > 0 && *item > prev {
            count += 1;
        }
        prev = *item;
    }
    return count;
}

fn main() {
    let nums: Vec<u16> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>();
    println!("{}", solve(&nums));
}

#[test]
fn test_example() {
    let nums: Vec<u16> = vec![
        199u16, 200u16, 208u16, 210u16, 200u16, 207u16, 240u16, 269u16, 260u16, 263u16,
    ];
    assert_eq!(solve(&nums), 7u16);
}
