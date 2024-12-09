use anyhow::Result;
use std::path::PathBuf;

use crate::util::{self, AocError};

struct Rule(usize, usize);

impl Rule {
    fn upheld(&self, left: usize, right: usize) -> bool {
        !(right == self.0 && left == self.1)
    }
}

fn parse_rules(input: &str) -> Result<Vec<Rule>> {
    input
        .split("\n")
        .filter(|s| s.len() != 0)
        .map(|s| {
            let (left, right) = s.split_once("|").ok_or(AocError::ParseError)?;
            let left = left.parse::<usize>()?;
            let right = right.parse::<usize>()?;
            Ok(Rule(left, right))
        })
        .collect()
}

fn parse_updates(input: &str) -> Result<Vec<Vec<usize>>> {
    input
        .split("\n")
        .filter(|s| s.len() != 0)
        .map(|s| {
            s.split(",")
                .filter(|s| s.len() != 0)
                .map(|s| Ok(s.parse::<usize>()?))
                .collect()
        })
        .collect()
}

fn separate_input(input: &str) -> Result<(Vec<Rule>, Vec<Vec<usize>>)> {
    let (rules, updates) = input.split_once("\n\n").ok_or(AocError::ParseError)?;
    let rules = parse_rules(rules)?;
    let updates = parse_updates(updates)?;
    Ok((rules, updates))
}

fn all_rules_upheld(left: usize, right: usize, rules: &Vec<Rule>) -> bool {
    rules.iter().map(|rule| rule.upheld(left, right)).all(|b| b)
}

fn update_upholds_rules(update: &[usize], rules: &Vec<Rule>) -> bool {
    update
        .windows(2)
        .flat_map(<&[usize; 2]>::try_from)
        .map(|&[left, right]| all_rules_upheld(left, right, rules))
        .all(|b| b)
}

fn quick_sort_update(update: &mut [usize], rules: &Vec<Rule>) {
    if update.len() < 2 {
        return;
    }

    let mut first_after_divide = 1;
    let length = update.len();
    for index in 1..length {
        if !all_rules_upheld(update[0], update[index], rules) {
            if first_after_divide < index {
                let (left, right) = update.split_at_mut(index);
                let left = &mut left[first_after_divide];
                let right = &mut right[0];
                std::mem::swap(left, right);
            }
            first_after_divide += 1;
        }
    }
    if first_after_divide > 1 {
        let (left, right) = update.split_at_mut(first_after_divide - 1);
        let left = &mut left[0];
        let right = &mut right[0];
        std::mem::swap(left, right);
    }

    // skip the correctly placed checked value
    let checked_pos = first_after_divide - 1;
    if checked_pos > 0 {
        quick_sort_update(&mut update[..checked_pos], rules);
    }
    if checked_pos < update.len() - 1 {
        quick_sort_update(&mut update[checked_pos + 1..], rules);
    }
}

pub fn run() -> Result<()> {
    println!("day 05");
    let path = PathBuf::from("./resources/day05.txt");
    let data = util::get_data_string(&path)?;
    let (rules, mut updates) = separate_input(&data)?;

    let mid_sum_correct = updates
        .iter()
        .filter(|u| update_upholds_rules(u, &rules))
        .map(|u| u[u.len() / 2])
        .sum::<usize>();
    println!("mid sum of correct updates: {mid_sum_correct}");

    let mid_sum_incorrect = updates
        .iter_mut()
        .filter(|u| !update_upholds_rules(u, &rules))
        .map(|u| {
            quick_sort_update(u, &rules);
            u
        })
        .map(|u| u[u.len() / 2])
        .sum::<usize>();
    println!("mid sum of incorrect updates: {mid_sum_incorrect}");

    Ok(())
}
