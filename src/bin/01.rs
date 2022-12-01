pub fn part_one(input: &str) -> Option<u32> {
    let mut cals: Vec<u32> = vec![];
    let mut current_sum = 0;
    for row in input.split("\n") {
        if row == "" {
            cals.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += row.parse::<u32>().unwrap();
        }
    }
    cals.push(current_sum);

    cals.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cals: Vec<u32> = vec![];
    let mut current_sum = 0;
    for row in input.split("\n") {
        if row == "" {
            cals.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += row.parse::<u32>().unwrap();
        }
    }
    cals.push(current_sum);

    cals.sort_unstable();
    Some(cals.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
