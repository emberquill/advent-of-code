use std::cmp::Ordering;

advent_of_code::solution!(5);

type Rules = Vec<[u8; 2]>;
type Updates = Vec<Vec<u8>>;

fn parse_input(input: &str) -> Option<(Rules, Updates)> {
    let mut rules: Rules = vec![];
    let mut updates: Updates = vec![];
    let mut finished_rules = false;
    for line in input.lines() {
        if line.is_empty() {
            finished_rules = true;
            continue;
        } else if finished_rules {
            updates.push(
                line.split(',')
                    .map(|page| page.parse::<u8>().unwrap())
                    .collect(),
            )
        } else {
            let rule: Vec<u8> = line
                .split('|')
                .map(|page| page.parse::<u8>().unwrap())
                .collect();
            rules.push([rule[0], rule[1]])
        }
    }
    Some((rules, updates))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input).unwrap();
    let mut sum: u32 = 0;
    for update in updates {
        if update.is_sorted_by(|a, b| {
            for rule in &rules {
                if &rule[0] == b && &rule[1] == a {
                    return false;
                }
            }
            true
        }) {
            sum += update[update.len() / 2] as u32;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input).unwrap();
    let mut sum: u32 = 0;
    for mut update in updates {
        if !update.is_sorted_by(|a, b| {
            for rule in &rules {
                if &rule[0] == b && &rule[1] == a {
                    return false;
                }
            }
            true
        }) {
            update.sort_by(|a, b| {
                for rule in &rules {
                    if &rule[0] == b && &rule[1] == a {
                        return Ordering::Less;
                    } else if &rule[0] == a && &rule[1] == b {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });
            sum += update[update.len() / 2] as u32;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
