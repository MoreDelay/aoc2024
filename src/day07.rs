use anyhow::Result;
use std::path::PathBuf;

use crate::util::{self, AocError};

struct Equation {
    target: usize,
    values: Vec<usize>,
}

fn generate_equations(input: &str) -> Result<Vec<Equation>> {
    let res = input
        .split("\n")
        .filter(|s| s.len() != 0)
        .map(|s| {
            let split_index = s.find(':').ok_or(AocError::ParseError)?;
            let (value, rest) = s.split_at(split_index);
            let value: usize = value.parse()?;
            let operands = &rest[1..].trim();
            let operands = operands
                .split_whitespace()
                .map(|o| Ok(o.parse()?))
                .collect::<Result<Vec<_>>>()?;
            assert_ne!(operands.len(), 0);
            Ok(Equation {
                target: value,
                values: operands,
            })
        })
        .collect::<Result<_>>()?;
    Ok(res)
}

enum NextOperators {
    Mul,
    Add,
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
    while !stack.is_empty() {
        let State {
            index,
            result,
            next_op,
        } = stack.pop().expect("stack not empty");
        let next_index = index + 1;
        if next_index == values.len() {
            if result == *target {
                return true;
            } else {
                continue;
            }
        }
        let next_value = values[next_index];
        match next_op {
            NextOperators::Mul => {
                let next_result = result * next_value;
                stack.push(State {
                    index,
                    result,
                    next_op: NextOperators::Add,
                });
                stack.push(State {
                    index: next_index,
                    result: next_result,
                    next_op: NextOperators::Mul,
                });
            }
            NextOperators::Add => {
                let next_result = result + next_value;
                stack.push(State {
                    index,
                    result,
                    next_op: NextOperators::Done,
                });
                stack.push(State {
                    index: next_index,
                    result: next_result,
                    next_op: NextOperators::Mul,
                });
            }
            NextOperators::Done => (),
        };
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

pub fn run() -> Result<()> {
    println!("day 07");
    let path = PathBuf::from("./resources/day07.txt");
    let data = util::get_data_string(&path)?;
    let equations = generate_equations(&data)?;
    let result = get_total_calibration_result(&equations);
    println!("total calibration result: {result}");
    Ok(())
}
