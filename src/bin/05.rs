fn parse_stacks(input: &str) -> Option<Vec<Vec<char>>> {
    let mut reversed = input.lines().rev();
    let numbers = reversed.next();
    let no = numbers.unwrap().replace(" ", "").len();
    let mut output: Vec<Vec<char>> = vec![];
    for _ in 0..no {
        output.push(vec![]);
    }
    for row in reversed {
        for i in 0..no {
            if row.chars().nth(1 + i * 4).unwrap() != ' ' {
                output[i].push(row.chars().nth(1 + i * 4).unwrap());
            }
        }
    }
    println!("{:?}", output);
    Some(output)
}

pub fn part_one(input: &str) -> Option<String> {
    let input_s: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = parse_stacks(input_s[0]).unwrap();

    for m in input_s[1].lines() {
        let instruction: Vec<&str> = m.split_ascii_whitespace().collect();
        let amount: u32 = instruction[1].parse().unwrap();
        let from: usize = instruction[3].parse::<usize>().unwrap() - 1;
        let to: usize = instruction[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let temp = stacks[from].pop().unwrap();
            stacks[to].push(temp);
        }
    }
    let mut output = String::new();
    for mut stack in stacks {
        output.push(stack.pop().unwrap())
    }
    println!("{:?}", output);
    Some(output)
}

pub fn part_two(input: &str) -> Option<String> {
    let input_s: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = parse_stacks(input_s[0]).unwrap();

    for m in input_s[1].lines() {
        let instruction: Vec<&str> = m.split_ascii_whitespace().collect();
        let amount: u32 = instruction[1].parse().unwrap();
        let from: usize = instruction[3].parse::<usize>().unwrap() - 1;
        let to: usize = instruction[5].parse::<usize>().unwrap() - 1;

        let mut temp: Vec<char> = vec![];

        for _ in 0..amount {
            temp.push(stacks[from].pop().unwrap());
        }
        for _ in 0..amount {
            stacks[to].push(temp.pop().unwrap());
        }
    }
    let mut output = String::new();
    for mut stack in stacks {
        output.push(stack.pop().unwrap())
    }
    println!("{:?}", output);
    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
