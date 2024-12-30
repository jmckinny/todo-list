use crate::TodoError;

pub fn get_index(args: &[String]) -> Result<usize, TodoError> {
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
