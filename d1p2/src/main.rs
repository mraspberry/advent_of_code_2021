use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

// answer is 1706

fn read_to_vec<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    let mut v: Vec<u32> = vec![];
    for line in br.lines() {
        v.push(
            line?
                .trim()
                .parse::<u32>()
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
        );
    }
    Ok(v)
}
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
    let file = File::open("input.txt").unwrap();
    let nums: Vec<u32> = read_to_vec(file).unwrap();
    println!("{}", solve(&nums));
}

#[test]
fn test_example() {
    let nums: Vec<u32> = vec![
        199u32, 200u32, 208u32, 210u32, 200u32, 207u32, 240u32, 269u32, 260u32, 263u32,
    ];
    assert_eq!(solve(&nums), 5u32);
}
