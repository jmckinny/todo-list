use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn import_todos() -> Vec<String> {
    search_file("src/main.rs")
}

fn search_file(path: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(path).unwrap());
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
    result
}
