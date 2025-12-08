advent_of_code::solution!(6);

#[derive(Clone, Copy)]
enum Op {
    Plus,
    Multiply,
}

impl TryFrom<&str> for Op {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Plus),
            "*" => Ok(Self::Multiply),
            other => Err(format!("Expected + or *, got {}", other)),
        }
    }
}

struct Problem {
    op: Op,
    inputs: Vec<u64>,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.op {
            Op::Plus => self.inputs.iter().sum(),
            Op::Multiply => self.inputs.iter().product(),
        }
    }

    fn solve_reverse(&self) -> u64 {}
}

fn parse_input(input: &str) -> Vec<Problem> {
    let operators = input
        .lines()
        .rev()
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|symbol| Op::try_from(symbol).ok())
        .collect::<Vec<Op>>();

    let problem_lines = input
        .lines()
        .rev()
        .skip(1)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|i| i.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut problems = vec![];

    for i in 0..problem_lines[0].len() {
        let mut inputs = vec![];
        for j in 0..problem_lines.len() {
            inputs.push(problem_lines[j][i]);
        }
        let problem = Problem {
            inputs,
            op: operators[i],
        };
        problems.push(problem);
    }

    problems
}

pub fn part_one(input: &str) -> Option<u64> {
    let problems = parse_input(input);
    let res = problems.iter().map(|p| p.solve()).sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problems = parse_input(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
