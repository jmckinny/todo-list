use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
pub fn import_todos(dir: impl AsRef<Path>) -> Result<Vec<String>, std::io::Error> {
    let mut result = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_type = entry.file_type()?;
        if entry_type.is_file() {
            result.append(&mut search_file(entry.path().to_str().unwrap())?);
        }
    }
    Ok(result)
}

fn search_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(path)?);
    for (i, line) in reader.lines().enumerate() {
        if let Ok(txt) = line {
            if let Some(start) = txt.find("TODO:") {
                result.push(format!(
                    "{} {}:{}",
                    txt.chars().skip(start).collect::<String>(),
                    path,
                    i
                ));
            }
        }
    }
    Ok(result)
}
