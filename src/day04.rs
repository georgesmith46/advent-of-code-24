const INPUT: &str = include_str!("input/day04.txt");
const CHAR_PATTERN: [char; 3] = ['M', 'A', 'S'];

fn check_n(x: usize, mut y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        y = y.checked_sub(1).unwrap_or(y);
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_ne(mut x: usize, mut y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        x += 1;
        y = y.checked_sub(1).unwrap_or(y);
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_e(mut x: usize, y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        x += 1;
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_se(mut x: usize, mut y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        x += 1;
        y += 1;
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_s(x: usize, mut y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        y += 1;
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_sw(mut x: usize, mut y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        x = x.checked_sub(1).unwrap_or(x);
        y += 1;
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_w(mut x: usize, y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        x = x.checked_sub(1).unwrap_or(x);
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_nw(mut x: usize, mut y: usize, input: &[Vec<char>]) -> bool {
    CHAR_PATTERN.iter().all(|expected_c| {
        x = x.checked_sub(1).unwrap_or(x);
        y = y.checked_sub(1).unwrap_or(y);
        input
            .get(y)
            .is_some_and(|line| line.get(x).is_some_and(|c| c == expected_c))
    })
}

fn check_adjacent(x: usize, y: usize, input: &[Vec<char>]) -> bool {
    check_n(x, y, input)
        || check_ne(x, y, input)
        || check_e(x, y, input)
        || check_se(x, y, input)
        || check_s(x, y, input)
        || check_sw(x, y, input)
        || check_w(x, y, input)
        || check_nw(x, y, input)
}

pub fn part_one() {
    let input = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let output = input.iter().enumerate().fold(0, |line_acc, (y, line)| {
        line_acc
            + line.iter().enumerate().fold(0, |char_acc, (x, &c)| {
                char_acc
                    + if c == 'X' && check_adjacent(x, y, &input) {
                        1
                    } else {
                        0
                    }
            })
    });
    println!("{output}");
}

pub fn part_two() {}
