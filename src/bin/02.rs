fn calculate_score(x: &char, y: &char) -> u32 {
    // X > Rock 1
    // Y > Paper 2 
    // Z > Siccor 3 
    match (x, y) {
        ('A', 'X') => 4,
        ('B', 'Y') => 5,
        ('C', 'Z') => 6,
        ('A', 'Y') => 8,
        ('B', 'Z') => 9,
        ('C', 'X') => 7,
        (_, 'X') => 1,
        (_, 'Y') => 2,
        (_, 'Z') => 3,
        _ => 0
    }
}

fn calculate_score2(x: &char, y: &char) -> u32 {
    // X > Win
    // Y > Draw
    // Z > Lose
    match (x, y) {
        ('A', 'X') => 6 + 2,
        ('B', 'X') => 3 + 2,
        ('C', 'X') => 2,

        ('A', 'Y') => 3 + 1,
        ('B', 'Y') => 1,
        ('C', 'Y') => 6 +1,

        ('A', 'Z') =>  3,
        ('B', 'Z') => 6 + 3,
        ('C', 'Z') => 3 + 3,
        _ => 0
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let l = input.lines();
    let score_list: Vec<u32> = l.map(|s| {
        let mut split = s.split(' ');
        calculate_score(
            &split.next().unwrap().chars().next().unwrap(), 
            &split.next().unwrap().chars().next().unwrap()
        )
    }
    ).collect();
    print!("{:?}", score_list);
    Some(score_list.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let l = input.lines();
    let score_list: Vec<u32> = l.map(|s| {
        let mut split = s.split(' ');
        calculate_score2(
            &split.next().unwrap().chars().next().unwrap(), 
            &split.next().unwrap().chars().next().unwrap()
        )
    }
    ).collect();
    print!("{:?}", score_list);
    Some(score_list.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
