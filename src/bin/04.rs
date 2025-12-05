advent_of_code::solution!(4);

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Roll,
    Empty,
}

impl TryFrom<char> for Cell {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Self::Empty),
            '@' => Ok(Self::Roll),
            other => Err(other),
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

    fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.cells.get(row).and_then(|r| r.get(col))
    }

    fn neighbor(&self, row: usize, col: usize, dr: isize, dc: isize) -> Option<&Cell> {
        let nrow = row.checked_add_signed(dr)?;
        let ncol = col.checked_add_signed(dc)?;
        self.get(nrow, ncol)
    }

    fn can_forklift_access(&self, row: usize, col: usize) -> bool {
        let adjacent_rolls = DIRECTIONS
            .iter()
            .filter(|&&(dr, dc)| self.neighbor(row, col, dr, dc) == Some(&Cell::Roll))
            .count();

        adjacent_rolls < 4
    }

    fn remove_roll(&mut self, row: usize, col: usize) {
        self.cells[row][col] = Cell::Empty;
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize, &Cell)> {
        self.cells.iter().enumerate().flat_map(|(row, cells)| {
            cells
                .iter()
                .enumerate()
                .map(move |(col, cell)| (row, col, cell))
        })
    }

    fn accessible_rolls(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.iter().filter_map(|(row, col, cell)| {
            (*cell == Cell::Roll && self.can_forklift_access(row, col)).then_some((row, col))
        })
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                let c = match cell {
                    Cell::Empty => '.',
                    Cell::Roll => '@',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input);
    let count = grid.accessible_rolls().count() as u64;
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_input(input);
    let mut total_count: u64 = 0;

    loop {
        let to_remove: Vec<_> = grid.accessible_rolls().collect();
        if to_remove.is_empty() {
            break;
        }

        total_count += to_remove.len() as u64;
        for (row, col) in to_remove {
            grid.remove_roll(row, col);
        }
    }

    Some(total_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
