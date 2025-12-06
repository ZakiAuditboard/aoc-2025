use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn ranges_from_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once("-").unwrap();
            let from = from.parse::<u64>().unwrap();
            let to = to.parse::<u64>().unwrap();
            from..=to
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let ranges = ranges_from_input(ranges_str);

    let result = ingredients_str
        .lines()
        .map(|ingredient| ingredient.parse::<u64>().unwrap())
        .filter(|i| ranges.iter().any(|range| range.contains(&i)))
        .count();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges_from_input(ranges);

    ranges.sort_by_key(|range| *range.start());

    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();

    for range in ranges {
        if merged.is_empty() || merged.last().unwrap().end() < range.start() {
            merged.push(range);
        } else {
            let last_merged = merged.last().unwrap();
            let ending = std::cmp::max(last_merged.end(), range.end());
            let len = merged.len();
            merged[len - 1] = *last_merged.start()..=*ending;
        }
    }

    let mut count = 0;

    for range in merged {
        count += range.end() - range.start() + 1;
    }

    Some(count as usize)
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
        assert_eq!(result, Some(14));
    }
}
