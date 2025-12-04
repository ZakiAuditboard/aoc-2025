advent_of_code::solution!(2);

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if len % 2 != 0 {
        return false;
    }

    let first_half = &id_str[0..len / 2];
    let second_half = &id_str[len / 2..];

    first_half == second_half
}

fn is_invalid_id_2(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    for i in 1..=len / 2 {
        if len % i != 0 {
            continue;
        }

        let first = &id_str[0..i];
        let mut all_match = true;
        for chunk in id_str.as_bytes().chunks(i) {
            if chunk != first.as_bytes() {
                all_match = false;
                break;
            }
        }

        if all_match {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .split(",")
        .flat_map(|range| {
            let (first, second) = range.split_once('-').unwrap();
            let first = first.parse::<u64>().unwrap();
            let second = second.parse::<u64>().unwrap();

            (first..=second).filter(|id| is_invalid_id(*id))
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .split(",")
        .flat_map(|range| {
            let (first, second) = range.split_once('-').unwrap();
            let first = first.parse::<u64>().unwrap();
            let second = second.parse::<u64>().unwrap();

            (first..=second).filter(|id| is_invalid_id_2(*id))
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_invalid_id_2() {
        let inputs: [u64; 13] = [
            11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824,
            2121212121,
        ];
        for input in inputs {
            if is_invalid_id_2(input) {
                println!("{input} passed");
                assert_eq!(true, true);
            } else {
                println!("{input} failed");
                assert_eq!(true, false);
            }
        }
    }
}
