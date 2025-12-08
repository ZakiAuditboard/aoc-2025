advent_of_code::solution!(7);
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
    Start,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '^' => Self::Splitter,
            'S' => Self::Start,
            _ => panic!(),
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
            .map(|line| line.chars().map(|c| Cell::from(c)).collect())
            .collect();

        Self { cells }
    }

    fn ncols(&self) -> usize {
        self.cells[0].len()
    }

    fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
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
            .find(|(_, _, cell)| matches!(cell, Cell::Start))
            .unwrap();

        (row, col)
    }

    fn go_down(&self, row: usize, col: usize, cache: &mut HashSet<(usize, usize)>) -> u64 {
        if col >= self.ncols() || !cache.insert((row, col)) {
            return 0;
        }

        let nrow = row + 1;

        match self.get(nrow, col) {
            Some(Cell::Splitter) => {
                let left = col
                    .checked_sub(1)
                    .map_or(0, |c| self.go_down(nrow, c, cache));
                let right = self.go_down(nrow, col + 1, cache);
                1 + left + right
            }
            Some(Cell::Empty) => self.go_down(nrow, col, cache),
            Some(Cell::Start) => 0,
            None => 0,
        }
    }

    // we need the cache here to memoize the paths
    // the keys are the positions (i.e. a position starting from row 1, col 5)
    // values are the number of paths from that position, to prevent recalculating and exponential complexity
    fn timelines_count(
        &self,
        row: usize,
        col: usize,
        cache: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        if col >= self.ncols() {
            return 0;
        }

        if let Some(&cached) = cache.get(&(row, col)) {
            return cached;
        }

        let nrow = row + 1;

        let result = match self.get(nrow, col) {
            Some(Cell::Splitter) => {
                let left = col
                    .checked_sub(1)
                    .map_or(0, |c| self.timelines_count(nrow, c, cache));
                let right = self.timelines_count(nrow, col + 1, cache);
                left + right
            }
            Some(Cell::Empty | Cell::Start) => self.timelines_count(nrow, col, cache),
            None => 1,
        };

        cache.insert((row, col), result);

        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input);
    let (row, col) = grid.find_start();

    Some(grid.go_down(row, col, &mut HashSet::new()))
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input);
    let (row, col) = grid.find_start();

    Some(grid.timelines_count(row, col, &mut HashMap::new()))
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
