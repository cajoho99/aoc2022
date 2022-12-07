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
    let games = input.lines();

    let results: Vec<u32> = games
        .map(|g| {
            let first = g.chars().nth(0).unwrap();
            let second = g.chars().nth(2).unwrap();

            match (first, second) {
                ('A', 'Z') => 8, // Paper
                ('B', 'Z') => 9, // Sten
                ('C', 'Z') => 7, // Sten

                ('A', 'Y') => 4,
                ('B', 'Y') => 5,
                ('C', 'Y') => 6,

                ('A', 'X') => 3, // Sciccors
                ('B', 'X') => 1, // Rock
                ('C', 'X') => 2, // Paper
                _ => 0,
            }
        })
        .collect();

    println!("{:?}", results);
    if results.contains(&0) {
        println!("Contains zero");
        return None;
    };
    return Some(results.iter().sum());

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
