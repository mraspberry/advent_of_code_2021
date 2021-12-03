use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

// answer is 1676

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
    let mut prev: u32 = 0;
    let mut count: u32 = 0;
    for item in items.into_iter() {
        if prev > 0 && *item > prev {
            count += 1;
        }
        prev = *item;
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
    assert_eq!(solve(&nums), 7u32);
}
