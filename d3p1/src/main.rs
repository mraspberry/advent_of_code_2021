use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Column {
    zeros: u32,
    ones: u32
}

impl Column {
    fn new() -> Column {
        Column { zeros: 0u32, ones: 0u32 }
    }

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

fn process_readings(readings: String) -> HashMap<usize, Column> {
    let mut columns: HashMap<usize, Column> = HashMap::new();
    for line in readings.lines() {
//        dbg!(line);
        if line.trim().is_empty() {
            continue
        }
        for (i, item) in line.chars().enumerate() {
            let mut column = columns.entry(i).or_insert(Column::new());
            match item {
                '1' => column.ones += 1,
                '0' => column.zeros += 1,
                _ => continue
            };
        }
    }
    return columns
}
fn solve(readings: String) -> u32 {
    let columns = process_readings(readings);
//    dbg!(&columns);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in {0..columns.len()}.into_iter() {
//        dbg!(i);
        gamma.push_str(columns[&i].most_common());
//        dbg!(&gamma);
        epsilon.push_str(columns[&i].least_common());
//        dbg!(&epsilon);
    }
//    dbg!(&gamma);
//    dbg!(&epsilon);
    u32::from_str_radix(gamma.as_str(), 2).unwrap() * u32::from_str_radix(epsilon.as_str(), 2).unwrap()
}
fn main() {
    let readings = fs::read_to_string("input.txt").expect("Unable to read input.txt");
    println!("{}", solve(readings));
}

#[test]
fn test_example() {
    let ans: u32 = 198;
    let input: Vec<&str> = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];
    assert_eq!(ans, solve(input.join("\n")));
}
