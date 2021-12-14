#[macro_use]
extern crate derive_new;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, new)]
struct Column {
    #[new(default)]
    zeros: u32,
    #[new(default)]
    ones: u32,
}

impl Column {
    fn most_common(&self) -> &str {
        if self.zeros > self.ones {
            "0"
        } else {
            "1"
        }
    }

    fn least_common(&self) -> &str {
        if self.zeros < self.ones {
            "0"
        } else {
            "1"
        }
    }
}

fn process_readings(readings: &str) -> HashMap<usize, Column> {
    let mut columns: HashMap<usize, Column> = HashMap::new();
    for line in readings.lines() {
        //        dbg!(line);
        if line.trim().is_empty() {
            continue;
        }
        for (i, item) in line.chars().enumerate() {
            let mut column = columns.entry(i).or_insert(Column::new());
            match item {
                '1' => column.ones += 1,
                '0' => column.zeros += 1,
                _ => continue,
            };
        }
    }
    return columns;
}
fn solve(readings: &str) -> u32 {
    let columns = process_readings(readings);
    //    dbg!(&columns);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in { 0..columns.len() }.into_iter() {
        //        dbg!(i);
        gamma.push_str(columns[&i].most_common());
        //        dbg!(&gamma);
        epsilon.push_str(columns[&i].least_common());
        //        dbg!(&epsilon);
    }
    //    dbg!(&gamma);
    //    dbg!(&epsilon);
    u32::from_str_radix(gamma.as_str(), 2).unwrap()
        * u32::from_str_radix(epsilon.as_str(), 2).unwrap()
}
fn main() {
    let readings = include_str!("../input.txt");
    println!("{}", solve(readings));
}

#[test]
fn test_example() {
    let ans: u32 = 198;
    let input: Vec<&str> = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(ans, solve(input.join("\n").as_str()));
}
