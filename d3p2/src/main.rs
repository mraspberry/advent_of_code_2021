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
    fn most_common(&self) -> char {
        if self.zeros > self.ones {
            '0'
        } else {
            '1'
        }
    }

    fn least_common(&self) -> char {
        if self.ones < self.zeros {
            '1'
        } else {
            '0'
        }
    }
}

fn populate_columns(readings: &Vec<&str>) -> HashMap<usize, Column> {
    let mut columns: HashMap<usize, Column> = HashMap::new();
    for r in readings.into_iter() {
        if r.trim().is_empty() {
            continue;
        }
        for (i, item) in r.chars().enumerate() {
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

fn calc_ogenerator_rating(readings: &str) -> u32 {
    let mut readings = readings.split_whitespace().collect::<Vec<_>>();
    //    dbg!(&readings);
    let mut columns = populate_columns(&readings);
    let mut index: usize = 0;
    while readings.len() > 1 {
        let col = columns.get(&index).unwrap();
        //        dbg!(col);
        //        dbg!(index);
        let most_common = col.most_common();
        readings = readings
            .into_iter()
            .filter(|r| r.chars().nth(index).unwrap() == most_common)
            .collect();
        //        dbg!(&readings);
        index += 1;
        columns = populate_columns(&readings);
    }
    u32::from_str_radix(readings[0], 2).unwrap()
}

fn calc_scrubber_rating(readings: &str) -> u32 {
    let mut readings = readings.split_whitespace().collect::<Vec<_>>();
    //    dbg!(&readings);
    let mut columns = populate_columns(&readings);
    let mut index: usize = 0;
    while readings.len() > 1 {
        let col = columns.get(&index).unwrap();
        let least_common = col.least_common();
        readings = readings
            .into_iter()
            .filter(|r| r.chars().nth(index).unwrap() == least_common)
            .collect();
        //        dbg!(&readings);
        index += 1;
        columns = populate_columns(&readings);
    }
    u32::from_str_radix(readings[0], 2).unwrap()
}

fn solve(readings: &str) -> u32 {
    let ogenerator = calc_ogenerator_rating(&readings);
    let scrubber = calc_scrubber_rating(&readings);
    //    dbg!(ogenerator);
    //    dbg!(scrubber);
    ogenerator * scrubber
}

fn main() {
    let readings = include_str!("../input.txt");
    println!("{}", solve(readings));
}

#[test]
fn test_example() {
    let ans: u32 = 230;
    let input: Vec<&str> = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    assert_eq!(ans, solve(input.join("\n").as_str()));
}
