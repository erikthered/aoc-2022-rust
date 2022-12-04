pub fn part_one(input: &str) -> Option<u32> {
    let mut cal = 0;
    let mut max_cal = 0;
    for line in input.lines() {
        if line.is_empty() {
            if cal > max_cal {
                max_cal = cal
            }
            cal = 0
        } else {
            cal += line.parse::<u32>().unwrap()
        }
    }
    Some(max_cal)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cal = 0;
    let mut totals = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            totals.push(cal);
            cal = 0
        } else {
            cal += line.parse::<u32>().unwrap()
        }
    }
    totals.sort();
    let top3 = totals.iter().rev().take(3).sum();
    Some(top3)
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
        assert_eq!(part_two(&input), None);
    }
}
