#[derive(Debug, Clone, Copy)]
enum SubMovementDirection {
    Forward,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct SubPositionChange {
    direction: SubMovementDirection,
    units: u32
}

impl SubPositionChange {
    fn new(direction: SubMovementDirection, units: u32) -> SubPositionChange {
        SubPositionChange {
            direction,
            units
        }
    }
}

#[derive(Debug)]
struct Submarine {
    horizontal: u32,
    depth: u32,
}

impl Submarine {

    fn change_position(&mut self, change: SubPositionChange) {
        match change.direction {
            Forward => self.horizontal += change.units,
            Up => self.depth -= change.units,
            Down => self.depth += change.units,
        }
    }

    fn report(&self) -> u32 {
        self.horizontal * self.depth
    }

    fn new() -> Submarine {
        Submarine { horizontal: 0u32, depth: 0u32 }
    }
}

fn solve(movements: &Vec<SubPositionChange>) -> u32 {
    let mut sub = Submarine::new();
    for movement in movements.iter() {
        dbg!(movement);
        sub.change_position(*movement);
    }
    sub.report()
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_example() {
    let ans: u32 = 150;
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
