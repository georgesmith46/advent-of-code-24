const INPUT: &str = include_str!("input/day04.txt");
const CHAR_PATTERN: [char; 3] = ['M', 'A', 'S'];

fn get_char(x: i64, y: i64, input: &[Vec<char>]) -> Option<&char> {
    input.get(y as usize).and_then(|line| line.get(x as usize))
}

fn check_match(x: i64, y: i64, expected_c: &char, input: &[Vec<char>]) -> bool {
    get_char(x, y, input).is_some_and(|c| c == expected_c)
}

fn check_adjacent_one(x: i64, y: i64, input: &[Vec<char>]) -> usize {
    [
        [0, -1],
        [1, -1],
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
    ]
    .iter()
    .filter_map(|coords| {
        let mut x = x;
        let mut y = y;
        CHAR_PATTERN
            .iter()
            .all(|expected_c| {
                x += coords[0];
                y += coords[1];
                if x < 0 || y < 0 {
                    return false;
                }
                check_match(x, y, expected_c, input)
            })
            .then_some(true)
    })
    .count()
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
                    + if c == 'X' {
                        check_adjacent_one(x as i64, y as i64, &input)
                    } else {
                        0
                    }
            })
    });
    println!("{output}");
}

fn check_adjacent_two(x: i64, y: i64, input: &[Vec<char>]) -> bool {
    if x == 0 || y == 0 {
        return false;
    }
    matches!(
        (get_char(x - 1, y - 1, input), get_char(x + 1, y + 1, input)),
        (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M'))
    ) && matches!(
        (get_char(x + 1, y - 1, input), get_char(x - 1, y + 1, input)),
        (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M'))
    )
}

pub fn part_two() {
    let input = INPUT
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let output = input.iter().enumerate().fold(0, |line_acc, (y, line)| {
        line_acc
            + line.iter().enumerate().fold(0, |char_acc, (x, &c)| {
                char_acc
                    + if c == 'A' && check_adjacent_two(x as i64, y as i64, &input) {
                        1
                    } else {
                        0
                    }
            })
    });
    println!("{output}");
}
