use std::fs;

fn part_one(input: &str) -> u64 {
    1
}

fn part_two(_input: &str) -> u64 {
    1
}

pub fn run() {
    let input = fs::read_to_string("input/day5.txt").unwrap();
    let ans = part_one(&input);
    println!("Part 1: {}", ans);
    let ans = part_two(&input);
    println!("Part 2: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47\n";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 143);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 1);
    }
}