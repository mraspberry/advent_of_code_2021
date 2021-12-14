// answer is 1706

fn solve(items: &Vec<u32>) -> u32 {
    let mut prev_sum: u32 = 0;
    let mut count: u32 = 0;
    for item in items.windows(3) {
        let sum: u32 = item.into_iter().sum();
        if prev_sum > 0 && sum > prev_sum {
            count += 1;
        }
        prev_sum = sum;
    }
    return count;
}

fn main() {
    let nums: Vec<u32> = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    println!("{}", solve(&nums));
}

#[test]
fn test_example() {
    let nums: Vec<u32> = vec![
        199u32, 200u32, 208u32, 210u32, 200u32, 207u32, 240u32, 269u32, 260u32, 263u32,
    ];
    assert_eq!(solve(&nums), 5u32);
}
