fn process_rucksack(rucksack: &str) -> u32 {
    let (first, last) = rucksack.split_at(rucksack.len() / 2);
    for c in first.chars() {
        if last.contains(c) {
            let out: u32;
            if c.is_uppercase() {
                out = c as u32 - 0x41 + 27;
            } else {
                out = c as u32 - 0x60;
            }
            println!("Found value {0}, which was converted to {1}", c, out);
            return out;
        }
    }
    println!("Did not reach a conclusion");
    return 0;
}

fn check_tree_elfs(first: &str, middle: &str, last: &str) -> u32 {
    for c in first.chars() {
        if middle.contains(c) && last.contains(c) {
            let out: u32;
            if c.is_uppercase() {
                out = c as u32 - 0x41 + 27;
            } else {
                out = c as u32 - 0x60;
            }
            println!("Found value {0}, which was converted to {1}", c, out);
            return out;
        }
    }
    println!("Did not reach a conclusion");
    return 0;
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks = input.lines();
    Some(
        rucksacks
            .map(|rucksack| process_rucksack(rucksack))
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<&str> = input.lines().collect();

    let len = rucksacks.len();

    let mut sum: u32 = 0;

    for i in 0..len / 3 {
        sum += check_tree_elfs(rucksacks[3 * i], rucksacks[3 * i + 1], rucksacks[3 * i + 2]);
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
