const INPUT: &str = include_str!("input/day01.txt");

fn get_vecs() -> (Vec<u32>, Vec<u32>) {
    INPUT
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(left, right)| (left.trim(), right.trim()))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .fold(
            (Vec::new(), Vec::new()),
            |(mut acc_left, mut acc_right), (left, right)| {
                acc_left.push(left);
                acc_right.push(right);
                (acc_left, acc_right)
            },
        )
}

pub fn part_one() {
    let (mut left, mut right) = get_vecs();
    left.sort();
    right.sort();

    let output = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (&left, &right)| acc + left.abs_diff(right));
    println!("{output}");
}

pub fn part_two() {
    let (left, right) = get_vecs();

    let output: u32 = left
        .iter()
        .map(|x| x * right.iter().filter(|&y| x == y).count() as u32)
        .sum();
    println!("{output}");
}
