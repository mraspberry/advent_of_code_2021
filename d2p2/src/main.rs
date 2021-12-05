use std::fs;

#[derive(Debug, Clone, Copy)]
enum SubMovementDirection {
    Forward,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct SubPositionChange {
    direction: SubMovementDirection,
    units: u32,
}

impl SubPositionChange {
    fn new(direction: SubMovementDirection, units: u32) -> SubPositionChange {
        SubPositionChange { direction, units }
    }
}

#[derive(Debug)]
struct Submarine {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Submarine {
    fn change_position(&mut self, change: SubPositionChange) {
        match change.direction {
            SubMovementDirection::Forward => {
                self.horizontal += change.units;
                self.depth += self.aim * change.units;
            }
            SubMovementDirection::Up => self.aim -= change.units,
            SubMovementDirection::Down => self.aim += change.units,
        };
    }

    fn report(&self) -> u32 {
        self.horizontal * self.depth
    }

    fn new() -> Submarine {
        Submarine {
            horizontal: 0u32,
            depth: 0u32,
            aim: 0u32,
        }
    }
}

fn solve(movements: &Vec<SubPositionChange>) -> u32 {
    let mut sub = Submarine::new();
    for movement in movements.iter() {
        sub.change_position(*movement);
    }
    sub.report()
}

fn main() {
    let mut movements: Vec<SubPositionChange> = Vec::new();
    let contents = fs::read_to_string("input.txt").expect("Unable to open input.txt");
    for instruction in contents.lines() {
        if instruction.trim().is_empty() {
            continue;
        }
        let i = instruction
            .split_whitespace()
            .take(2)
            .collect::<Vec<&str>>();
        if let [d, u] = &i[..] {
            let units = u.parse::<u32>().unwrap();
            let direction = match *d {
                "forward" => SubMovementDirection::Forward,
                "up" => SubMovementDirection::Up,
                "down" => SubMovementDirection::Down,
                &_ => panic!("Unknown direction"),
            };
            movements.push(SubPositionChange::new(direction, units));
        }
    }
    println!("{}", solve(&movements));
}

#[test]
fn test_example() {
    let ans: u32 = 900;
    let movements = vec![
        SubPositionChange::new(SubMovementDirection::Forward, 5u32),
        SubPositionChange::new(SubMovementDirection::Down, 5u32),
        SubPositionChange::new(SubMovementDirection::Forward, 8u32),
        SubPositionChange::new(SubMovementDirection::Up, 3u32),
        SubPositionChange::new(SubMovementDirection::Down, 8u32),
        SubPositionChange::new(SubMovementDirection::Forward, 2u32),
    ];
    assert_eq!(ans, solve(&movements));
}
