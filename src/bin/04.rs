fn is_within(left: &str, right: &str) -> bool {
    let mut sleft = left.split('-');
    let mut sright = right.split('-');

    let (ll, lr) = (
        sleft.next().unwrap().parse::<u32>().unwrap(),
        sleft.next().unwrap().parse::<u32>().unwrap(),
    );

    let (rl, rr) = (
        sright.next().unwrap().parse::<u32>().unwrap(),
        sright.next().unwrap().parse::<u32>().unwrap(),
    );

    ll >= rl && lr <= rr
}

fn is_overlapping(left: &str, right: &str) -> bool {
    let mut sleft = left.split('-');
    let mut sright = right.split('-');

    let (ll, lr) = (
        sleft.next().unwrap().parse::<u32>().unwrap(),
        sleft.next().unwrap().parse::<u32>().unwrap(),
    );

    let (rl, rr) = (
        sright.next().unwrap().parse::<u32>().unwrap(),
        sright.next().unwrap().parse::<u32>().unwrap(),
    );

    ll >= rl && ll <= rr
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    let schedule_row = input.lines();
    for row in schedule_row {
        let mut pairs = row.split(',');
        let left = pairs.next().unwrap();
        let right = pairs.next().unwrap();

        if is_within(left, right) || is_within(right, left) {
            count += 1
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count: u32 = 0;
    let schedule_row = input.lines();
    for row in schedule_row {
        let mut pairs = row.split(',');
        let left = pairs.next().unwrap();
        let right = pairs.next().unwrap();

        if is_overlapping(left, right) || is_overlapping(right, left) {
            count += 1
        }
    }
    Some(count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
