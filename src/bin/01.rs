advent_of_code::solution!(1);

const TICKS_IN_DIAL: usize = 100;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Rotation {
    direction: Direction,
    distance: usize,
}

impl Rotation {
    fn from_string(input: &str) -> Self {
        let mut chars = input.chars();

        let direction = match chars.next().unwrap() {
            'L' => Direction::Left,
            'R' => Direction::Right,
            other => panic!("Expected L or R, got {other}"),
        };

        let distance = chars.collect::<String>().parse::<usize>().unwrap();

        Self {
            direction,
            distance,
        }
    }
}

struct Dial<const TICKS: usize> {
    position: usize,
}

impl<const TICKS: usize> Dial<TICKS> {
    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.position = (self.position + TICKS - 1) % TICKS;
            }
            Direction::Right => {
                self.position = (self.position + 1) % TICKS;
            }
        }
    }

    fn rotate(&mut self, rotation: Rotation) {
        let dist = rotation.distance % TICKS;

        for _ in 0..dist {
            self.step(rotation.direction);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut dial = Dial::<TICKS_IN_DIAL> { position: 50 };
    let mut count = 0;

    for rotation in input.lines().map(Rotation::from_string) {
        dial.rotate(rotation);

        if dial.position == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut dial = Dial::<TICKS_IN_DIAL> { position: 50 };
    let mut count = 0;

    for rotation in input.lines().map(Rotation::from_string) {
        for _ in 0..(rotation.distance) {
            dial.step(rotation.direction);

            if dial.position == 0 {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
