advent_of_code::solution!(4);

const DIRECTIONS: [(i16, i16); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

#[derive(PartialEq)]
enum Cell {
    Roll,
    Empty,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Roll,
            other => panic!("Expected '.' or '@', got {}", other),
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| line.chars().map(Cell::from_char).collect::<Vec<Cell>>())
            .collect();

        let rows = cells.len();
        let cols = cells[0].len();

        Self { cells, rows, cols }
    }

    // Assumes that the cell at (row, col) is a Roll, not Empty
    fn can_forklift_access(&self, row: usize, col: usize) -> bool {
        let adjacent_rolls: usize = DIRECTIONS
            .map(|(dx, dy)| {
                let nrow = row as i16 + dx;
                let ncol = col as i16 + dy;
                if nrow >= 0
                    && ncol >= 0
                    && nrow < self.rows as i16
                    && ncol < self.cols as i16
                    && self.cells[nrow as usize][ncol as usize] == Cell::Roll
                {
                    1
                } else {
                    0
                }
            })
            .iter()
            .sum();

        adjacent_rolls < 4
    }

    fn remove_roll(&mut self, row: usize, col: usize) {
        self.cells[row][col] = Cell::Empty;
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Empty => output.push('.'),
                    Cell::Roll => output.push('@'),
                }
            }
            output.push('\n')
        }

        write!(f, "{}", output)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::from_input(input);
    let mut count: u64 = 0;

    println!("{}", grid);

    for i in 0..grid.cells.len() {
        for j in 0..grid.cells[0].len() {
            if grid.cells[i][j] == Cell::Roll && grid.can_forklift_access(i, j) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::from_input(input);
    let mut count: u64 = 0;

    loop {
        let mut last_removed = 0;

        for i in 0..grid.cells.len() {
            for j in 0..grid.cells[0].len() {
                if grid.cells[i][j] == Cell::Roll && grid.can_forklift_access(i, j) {
                    last_removed += 1;
                    grid.remove_roll(i, j);
                }
            }
        }

        count += last_removed;
        if last_removed == 0 {
            break;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
