use std::{iter::Peekable, str::Chars};

const INPUT: &str = include_str!("input/day03.txt");

struct InputIterator(Peekable<Chars<'static>>);

impl InputIterator {
    fn next(&mut self) -> Option<char> {
        self.0.next()
    }

    fn next_number(&mut self) -> Option<u64> {
        let next = std::iter::from_fn(|| self.0.by_ref().next_if(|x| x.is_ascii_digit()))
            .collect::<String>()
            .parse::<u64>();
        next.ok().filter(|&x| x < 1000)
    }

    fn next_chars(&mut self, chars: &[char]) -> bool {
        chars.iter().all(|&x| self.0.next_if(|&c| x == c).is_some())
    }
}

pub fn part_one() {
    let mut iter = InputIterator(INPUT.chars().peekable());
    let mut output = 0;

    while let Some(c) = iter.next() {
        if c == 'm' && iter.next_chars(&['u', 'l', '(']) {
            if let Some(first_number) = iter.next_number() {
                if iter.next_chars(&[',']) {
                    if let Some(second_number) = iter.next_number() {
                        if iter.next_chars(&[')']) {
                            output += first_number * second_number;
                        }
                    }
                }
            }
        }
    }

    println!("{output}");
}

pub fn part_two() {
    let mut iter = InputIterator(INPUT.chars().peekable());
    let mut output = 0;
    let mut enable = true;

    while let Some(c) = iter.next() {
        if c == 'm' && iter.next_chars(&['u', 'l', '(']) {
            if let Some(first_number) = iter.next_number() {
                if iter.next_chars(&[',']) {
                    if let Some(second_number) = iter.next_number() {
                        if iter.next_chars(&[')']) && enable {
                            output += first_number * second_number;
                        }
                    }
                }
            }
        } else if c == 'd' && iter.next_chars(&['o']) {
            if iter.next_chars(&['n', '\'', 't', '(', ')']) {
                enable = false;
            } else if iter.next_chars(&['(', ')']) {
                enable = true;
            }
        }
    }

    println!("{output}");
}
