use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Parse Error")]
    ParseError,
    #[error("Value Error: {0}")]
    ValueError(String),
}

pub fn get_data_string(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut result = String::new();
    reader.read_to_string(&mut result)?;
    Ok(result)
}

pub fn get_data_fixed_columns<const C: usize>(path: &Path) -> Result<[Vec<usize>; C]> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result: [Vec<usize>; C] = vec![Vec::new(); C].try_into().unwrap();
    for line in reader.lines() {
        let parsed: Vec<Result<_>> = line?
            .split_whitespace()
            .map(|val| val.parse::<usize>().map_err(|e| e.into()))
            .collect();
        let parsed: Result<Vec<usize>> = parsed.into_iter().collect();
        let parsed = parsed?;
        assert_eq!(parsed.len(), C);
        for index in 0..C {
            result[index].push(parsed[index]);
        }
    }
    Ok(result)
}

pub fn get_data_rows(path: &Path) -> Result<Vec<Vec<usize>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let parsed: Vec<Result<_>> = line?
            .split_whitespace()
            .map(|val| val.parse::<usize>().map_err(|e| e.into()))
            .collect();
        let parsed: Result<Vec<usize>> = parsed.into_iter().collect();
        let parsed = parsed?;
        result.push(parsed);
    }
    Ok(result)
}

pub fn parse_tiles<T, F>(input: &str, mut parser: F) -> Result<Vec<Vec<T>>, AocError>
where
    F: FnMut(usize, usize, char) -> Result<T, AocError>,
{
    let mut width = None;
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(y, s)| {
            let row = s
                .chars()
                .enumerate()
                .map(|(x, c)| parser(x, y, c))
                .collect::<Result<Vec<_>, AocError>>()?;
            if *width.get_or_insert(row.len()) != row.len() {
                Err(AocError::ParseError)
            } else {
                Ok(row)
            }
        })
        .collect()
}

pub fn is_even(val: usize) -> bool {
    (val & 1) == 0
}
