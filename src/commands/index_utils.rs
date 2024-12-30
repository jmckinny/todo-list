use crate::TodoError;

pub enum IndexSelection {
    Single(usize),
    Range((usize, usize)),
}

pub fn get_index(args: &[String]) -> Result<IndexSelection, TodoError> {
    if args.get(2).unwrap_or(&String::new()).contains(",") {
        let range = get_index_range(args)?;
        Ok(IndexSelection::Range(range))
    } else {
        let index = get_single_index(args)?;
        Ok(IndexSelection::Single(index))
    }
}

pub fn get_single_index(args: &[String]) -> Result<usize, TodoError> {
    match args.get(2) {
        Some(index_str) => {
            let index: usize = index_str.parse()?;
            Ok(index)
        }
        None => {
            eprintln!("Usage: todo <command> <index>");
            std::process::exit(1);
        }
    }
}

pub fn get_index_range(args: &[String]) -> Result<(usize, usize), TodoError> {
    match args.get(2) {
        Some(indexs_str) => {
            let mut indexes = indexs_str.split(',');
            let start = indexes.next().unwrap_or_default().parse()?;
            let end = indexes.next().unwrap_or_default().parse()?;
            Ok((start, end))
        }
        None => {
            eprintln!("Usage: todo <command> <index>,<index>");
            std::process::exit(1);
        }
    }
}
