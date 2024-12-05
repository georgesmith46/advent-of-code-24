const INPUT: &str = include_str!("input/day02.txt");

#[derive(Copy, Clone)]
enum Pattern {
    Unknown,
    Increasing,
    Decreasing,
}

fn check_levels(line: &[i32]) -> bool {
    let mut pattern = Pattern::Unknown;
    !line.windows(2).any(|levels| {
        let curr_level = levels[0];
        let next_level = levels[1];
        let diff = curr_level.abs_diff(next_level);
        if !(1..=3).contains(&diff) {
            return true;
        }
        let next_pattern = if curr_level > next_level {
            Pattern::Decreasing
        } else {
            Pattern::Increasing
        };
        match (pattern, next_pattern) {
            (Pattern::Unknown, _) => {
                pattern = next_pattern;
            }
            (Pattern::Decreasing, Pattern::Increasing)
            | (Pattern::Increasing, Pattern::Decreasing) => {
                return true;
            }
            _ => {}
        }
        false
    })
}

pub fn part_one() {
    let output = INPUT
        .lines()
        .filter(|line| {
            let line = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            check_levels(&line)
        })
        .count();
    println!("{output}");
}

pub fn part_two() {
    let output = INPUT
        .lines()
        .filter(|line| {
            let line = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            check_levels(&line)
                || line
                    .iter()
                    .enumerate()
                    .any(|(i, _)| check_levels(&[&line[..i], &line[i + 1..]].concat()))
        })
        .count();
    println!("{output}");
}
