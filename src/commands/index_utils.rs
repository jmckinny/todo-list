use crate::{data::todo_list::TodoList, TodoError};

pub enum IndexSelection {
    Single(usize),
    Range((usize, usize)),
    List(Vec<usize>),
}

const RANGE_DELIM: char = '-';
const LIST_DELIM: char = ',';

pub fn get_index(todo_list: &TodoList, args: &[String]) -> Result<IndexSelection, TodoError> {
    let empty = String::new();
    let arg = args.get(2).unwrap_or(&empty);

    let num_count = arg.chars().filter(|c| c.is_ascii_digit()).count();
    let contains_range_delim = arg.contains(RANGE_DELIM);
    let contains_set_delim = arg.contains(LIST_DELIM);

    if num_count > 1 && contains_range_delim {
        let range = get_index_range(args)?;
        Ok(IndexSelection::Range(range))
    } else if num_count > 1 && contains_set_delim {
        let index_set = get_index_list(args)?;
        Ok(IndexSelection::List(index_set))
    } else if num_count > 0 {
        let index = get_single_index(args)?;
        Ok(IndexSelection::Single(index))
    } else {
        let index = get_term_index(args, todo_list)?;
        Ok(IndexSelection::Single(index))
    }
}

pub fn get_single_index(args: &[String]) -> Result<usize, TodoError> {
    match args.get(2) {
        Some(index_str) => {
            let index: usize = index_str.parse()?;
            Ok(index)
        }
        None => Err(TodoError::UsageError(format!(
            "Usage: todo <{}> <index>",
            args.get(1).map(String::as_str).unwrap_or("command")
        ))),
    }
}

pub fn get_index_range(args: &[String]) -> Result<(usize, usize), TodoError> {
    match args.get(2) {
        Some(indexs_str) => {
            let mut indexes = indexs_str.split(RANGE_DELIM);
            let start = indexes.next().unwrap_or_default().parse()?;
            let end = indexes.next().unwrap_or_default().parse()?;
            Ok((start, end))
        }
        None => Err(TodoError::UsageError(format!(
            "Usage: todo <{}> <start-end>",
            args.get(1).map(String::as_str).unwrap_or("command")
        ))),
    }
}

pub fn get_term_index(args: &[String], todo_list: &TodoList) -> Result<usize, TodoError> {
    const MAX_SCORE_ALLOWED: usize = 2; // TODO: take search term length into account
    let search_term = args
        .iter()
        .skip(2)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");
    if search_term.is_empty() {
        eprintln!("Usage: todo <command> <fuzzy_match>");
        std::process::exit(1);
    }
    let scores: Vec<usize> = todo_list
        .get_items()
        .iter()
        .map(|item| levenshtein_distance(&search_term, item.get_content()))
        .collect();

    let min_score: Option<(_, _)> = scores.iter().enumerate().min_by(|(_, a), (_, b)| a.cmp(b));

    match min_score {
        Some((index, score)) if *score < MAX_SCORE_ALLOWED => Ok(index + 1),
        _ => Err(TodoError::FuzzyMatchFailed(search_term.to_string())),
    }
}

fn get_index_list(args: &[String]) -> Result<Vec<usize>, TodoError> {
    match args.get(2) {
        Some(lst) => {
            let index_set: Result<Vec<_>, _> = lst
                .split(LIST_DELIM)
                .map(|num_str| num_str.parse())
                .collect();
            Ok(index_set?)
        }
        None => Err(TodoError::UsageError(format!(
            "Usage: todo <{}> <index,index,...>",
            args.get(1).map(String::as_str).unwrap_or("command")
        ))),
    }
}

fn levenshtein_distance(target: &str, other: &str) -> usize {
    let mut matrix = vec![vec![0_usize; target.len() + 1]; other.len() + 1];
    let target_len = target.chars().count();
    let other_len = other.chars().count();
    // Empty string sub-problems can just be solved by inserting i characters
    for (i, row) in matrix.iter_mut().enumerate().take(other_len + 1) {
        row[0] = i;
    }

    for j in 0..=target_len {
        matrix[0][j] = j
    }

    for j in 1..=target_len {
        for i in 1..=other_len {
            let substitue_cost =
                if target.chars().nth(j - 1).unwrap() == other.chars().nth(i - 1).unwrap() {
                    0
                } else {
                    1
                };

            matrix[i][j] = min3(
                matrix[i - 1][j] + 1,                  //deletion
                matrix[i][j - 1] + 1,                  //insertion
                matrix[i - 1][j - 1] + substitue_cost, //substitution
            );
        }
    }
    matrix[other_len][target_len]
}

fn min3(x: usize, y: usize, z: usize) -> usize {
    x.min(y.min(z))
}
