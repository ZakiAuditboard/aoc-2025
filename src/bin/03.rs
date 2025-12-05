advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|line| {
            let digits = line.bytes().map(|b| b - b'0').collect::<Vec<u8>>();
            let mut max: u64 = 0;

            for i in 0..digits.len() {
                for j in i + 1..digits.len() {
                    let number = 10 * digits[i] + digits[j];
                    max = std::cmp::max(number as u64, max);
                }
            }

            max
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|line| {
            let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
            let n = digits.len();
            let k = 12;

            let mut result: u64 = 0;
            let mut start = 0;

            for i in 0..k {
                let remaining_to_pick = k - i - 1;
                let end = n - remaining_to_pick;

                let mut max_idx = start;
                for j in start + 1..end {
                    if digits[j] > digits[max_idx] {
                        max_idx = j;
                    }
                }

                result = result * 10 + digits[max_idx] as u64;
                start = max_idx + 1;
            }

            result
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
