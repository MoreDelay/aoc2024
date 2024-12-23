use anyhow::Result;
use std::path::PathBuf;

use crate::util::{self, AocError};

struct Equation {
    target: usize,
    values: Vec<usize>,
}

fn generate_equations(input: &str) -> Result<Vec<Equation>> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let split_index = s.find(':').ok_or(AocError::ParseError)?;
            let (value, rest) = s.split_at(split_index);
            let value: usize = value.parse()?;
            let operands = &rest[1..].trim();
            let operands = operands
                .split_whitespace()
                .map(|o| Ok(o.parse()?))
                .collect::<Result<Vec<_>>>()?;
            if operands.is_empty() {
                return Err(AocError::ParseError.into());
            }
            Ok(Equation {
                target: value,
                values: operands,
            })
        })
        .collect::<Result<_>>()
}

enum NextOperators {
    Mul,
    Add,
    Concat,
    Done,
}

struct State {
    index: usize,
    result: usize,
    next_op: NextOperators,
}

// Depth First Search with backtracking when result gets too large
fn can_construct_equation(equation: &Equation) -> bool {
    let Equation { target, values } = equation;
    let mut stack = Vec::new();
    let initial_state = State {
        index: 0,
        result: equation.values[0],
        next_op: NextOperators::Mul,
    };
    stack.push(initial_state);
    while let Some(top) = stack.pop() {
        let State {
            index,
            result,
            next_op,
        } = top;

        let next_index = index + 1;
        if next_index == values.len() {
            if result == *target {
                return true;
            } else {
                continue;
            }
        }
        if result > *target {
            continue;
        }
        let next_value = values[next_index];
        let next_result = match next_op {
            NextOperators::Mul => {
                stack.push(State {
                    index,
                    result,
                    next_op: NextOperators::Add,
                });
                result * next_value
            }
            NextOperators::Add => {
                stack.push(State {
                    index,
                    result,
                    next_op: NextOperators::Done,
                });
                result + next_value
            }
            NextOperators::Done => continue,
            NextOperators::Concat => unreachable!(),
        };

        stack.push(State {
            index: next_index,
            result: next_result,
            next_op: NextOperators::Mul,
        });
    }
    false
}

// Depth First Search with backtracking when result gets too large
fn can_construct_equation_with_concat(equation: &Equation) -> bool {
    let Equation { target, values } = equation;
    let mut stack = Vec::new();
    let initial_state = State {
        index: 0,
        result: equation.values[0],
        next_op: NextOperators::Mul,
    };
    stack.push(initial_state);
    while let Some(top) = stack.pop() {
        let State {
            index,
            result,
            next_op,
        } = top;

        let next_index = index + 1;
        if next_index == values.len() {
            if result == *target {
                return true;
            } else {
                continue;
            }
        }
        if result > *target {
            continue;
        }
        let next_value = values[next_index];
        let next_result = match next_op {
            NextOperators::Mul => {
                stack.push(State {
                    index,
                    result,
                    next_op: NextOperators::Add,
                });
                result * next_value
            }
            NextOperators::Add => {
                stack.push(State {
                    index,
                    result,
                    next_op: NextOperators::Concat,
                });
                result + next_value
            }
            NextOperators::Concat => {
                stack.push(State {
                    index,
                    result,
                    next_op: NextOperators::Done,
                });
                let mut concatted = result.to_string();
                concatted.push_str(&next_value.to_string());
                concatted.parse().unwrap()
            }
            NextOperators::Done => continue,
        };
        stack.push(State {
            index: next_index,
            result: next_result,
            next_op: NextOperators::Mul,
        });
    }
    false
}

fn get_total_calibration_result(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|e| can_construct_equation(e))
        .map(|e| e.target)
        .sum()
}

fn get_total_calibration_result_with_concat(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|e| can_construct_equation_with_concat(e))
        .map(|e| e.target)
        .sum()
}

pub fn run() -> Result<()> {
    println!("day 07");
    let path = PathBuf::from("./resources/day07.txt");
    let data = util::get_data_string(&path)?;
    let equations = generate_equations(&data)?;
    let result = get_total_calibration_result(&equations);
    println!("total calibration result: {result}");
    let result = get_total_calibration_result_with_concat(&equations);
    println!("total calibration result with concat: {result}");
    Ok(())
}
