use std::collections::{HashSet, HashMap};



struct Directory<'a> {
    parent: Option<&'a Directory<'a>>,
    dir: HashMap<String,&'a Directory<'a>>,
    files: Vec<u32>
}
    
impl Directory<'_> {
    fn new(parent: Option<&Directory>) -> Self {
        Directory { parent, dir: HashMap::new(), files: vec![]}
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    
    let dir = Directory::new(None);
    let root = &dir;
    let curr: &mut Directory = &dir;
    for line in input.lines() {
        if line == "$ cd .." {
            println!("cd ..");
            curr = curr.parent.as_ref().unwrap();
        }
        else if line.starts_with("$ cd ") {
            println!("moving to other dir");
            let dir_name = line.replace("$ cd ", "");
            curr = curr.dir[&dir_name];
        }
        else if line == "$ ls" {
            println!("do nothing ls");
        }
        else if line.starts_with("dir ") {
            println!("Add dir");
            let dir_name = line.replace("$ cd ", "");
            let new_dir = Directory::new(Some(curr));
            curr.dir.insert(dir_name, &new_dir).unwrap();
        }
        else {
            println!("file added.");
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
