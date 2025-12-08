advent_of_code::solution!(7);
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, PartialOrd)]
enum Cell {
    Empty,
    Splitter,
    Start,
}

impl TryFrom<char> for Cell {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '^' => Ok(Self::Splitter),
            'S' => Ok(Self::Start),
            _ => Err(String::from("Unknown character")),
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Cell::try_from(c).expect("valid cell character"))
                    .collect()
            })
            .collect();

        Self { cells }
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> {
        self.cells.iter().enumerate().flat_map(|(row, cells)| {
            cells
                .iter()
                .enumerate()
                .map(move |(col, cell)| (row, col, cell))
        })
    }

    fn find_start(&self) -> (usize, usize) {
        let (row, col, _) = self
            .iter()
            .find(|(_, _, cell)| **cell == Cell::Start)
            .unwrap();

        (row, col)
    }

    fn go_down(&self, row: usize, col: isize, cache: &mut HashSet<(usize, usize)>) -> u64 {
        if col < 0 || col as usize >= self.cells[0].len() || cache.contains(&(row, col as usize)) {
            return 0;
        }

        cache.insert((row, col as usize));

        let nrow = row + 1;

        if nrow >= self.cells.len() {
            return 0;
        }

        if self.cells[nrow][col as usize] == Cell::Splitter {
            return 1 + self.go_down(nrow, col - 1, cache) + self.go_down(nrow, col + 1, cache);
        }

        if self.cells[nrow][col as usize] == Cell::Empty {
            return self.go_down(nrow, col, cache);
        }

        0
    }

    // we need the cache here to memoize the paths
    // the keys are the positions (i.e. a position starting from row 1, col 5)
    // values are the number of paths from that position, to prevent recalculating and exponential complexity
    fn timelines_count(
        &self,
        row: usize,
        col: isize,
        cache: &mut HashMap<(usize, isize), u64>,
    ) -> u64 {
        if col < 0 || col as usize >= self.cells[0].len() {
            return 0;
        }

        if let Some(&cached) = cache.get(&(row, col)) {
            return cached;
        }

        let nrow = row + 1;

        if nrow >= self.cells.len() {
            return 1;
        }

        let result = match &self.cells[nrow][col as usize] {
            Cell::Splitter => {
                self.timelines_count(nrow, col - 1, cache)
                    + self.timelines_count(nrow, col + 1, cache)
            }
            Cell::Empty | Cell::Start => self.timelines_count(nrow, col, cache),
        };

        cache.insert((row, col), result);

        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input);
    let (row, col) = grid.find_start();

    Some(grid.go_down(row, col as isize, &mut HashSet::new()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input);
    let (row, col) = grid.find_start();

    Some(grid.timelines_count(row, col as isize, &mut HashMap::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
