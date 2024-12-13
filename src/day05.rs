use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input/day05.txt");

fn get_order(rules: &[(usize, usize)], pages_to_include: &[usize]) -> Vec<usize> {
    let rules: HashMap<usize, HashSet<usize>> = rules
        .iter()
        .filter(|(left, _)| pages_to_include.contains(left))
        .fold(HashMap::new(), |mut acc, (left, right)| {
            acc.entry(*left).or_default().insert(*right);
            acc
        });

    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    let mut queue: Vec<usize> = Vec::new();
    let mut order: Vec<usize> = Vec::new();

    for (vertex, _) in rules.iter() {
        in_degree.insert(*vertex, 0);
    }

    for (_, neighbors) in rules.iter() {
        for neighbor in neighbors {
            in_degree
                .entry(*neighbor)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    for (vertex, degree) in in_degree.iter() {
        if *degree == 0 {
            queue.push(*vertex);
        }
    }

    while !queue.is_empty() {
        let vertex = queue.remove(0);
        order.push(vertex);

        if rules.contains_key(&vertex) {
            for neighbor in rules.get(&vertex).unwrap() {
                in_degree
                    .entry(*neighbor)
                    .and_modify(|x| *x = x.checked_sub(1).unwrap_or(*x));
                if in_degree.get(neighbor).is_some_and(|x| *x == 0) {
                    queue.push(*neighbor);
                }
            }
        }
    }

    order
}

pub fn part_one() {
    let (rules, pages) = INPUT.split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = rules
        .lines()
        .map(|rule| rule.split_once('|').unwrap())
        .map(|(left, right)| {
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let output = pages.lines().fold(0, |acc, line| {
        let pages = line
            .split(',')
            .map(|page| page.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let pages_len = pages.len();
        let order = get_order(&rules, &pages);
        acc + if pages.is_sorted_by_key(|x| {
            order
                .iter()
                .enumerate()
                .find(|(_, page)| *page == x)
                .unwrap_or((usize::MAX, &0))
                .0
        }) {
            pages[pages_len / 2]
        } else {
            0
        }
    });

    println!("{output}");
}

pub fn part_two() {
    let (rules, pages) = INPUT.split_once("\n\n").unwrap();
    let rules: Vec<(usize, usize)> = rules
        .lines()
        .map(|rule| rule.split_once('|').unwrap())
        .map(|(left, right)| {
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .collect();

    let output = pages.lines().fold(0, |acc, line| {
        let mut pages = line
            .split(',')
            .map(|page| page.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let pages_len = pages.len();
        let order = get_order(&rules, &pages);
        let sort_fn = |x: &usize| {
            order
                .iter()
                .enumerate()
                .find(|(_, page)| *page == x)
                .unwrap_or((usize::MAX, &0))
                .0
        };
        acc + if !pages.is_sorted_by_key(sort_fn) {
            pages.sort_by_key(sort_fn);
            pages[pages_len / 2]
        } else {
            0
        }
    });

    println!("{output}");
}
