use std::io::{self, Write};

pub fn prompt_create_new_todo_list() -> bool {
    yes_no_prompt("No todo file found, create one?", DefaultResponse::Yes)
}

pub fn prompt_remove_item(count: usize) -> bool {
    let question = format!("Remove '{}' items?", count);
    yes_no_prompt(&question, DefaultResponse::No)
}

enum DefaultResponse {
    Yes,
    No,
}

fn yes_no_prompt(question_text: &str, default: DefaultResponse) -> bool {
    let mut input = String::new();
    match default {
        DefaultResponse::Yes => print!("{} [Y/n] ", question_text),
        DefaultResponse::No => print!("{} [y/N] ", question_text),
    }
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading from STDIN");
    let input = input.trim().to_lowercase();
    match input.as_str() {
        "y" => true,
        "n" => false,
        _ => match default {
            DefaultResponse::Yes => true,
            DefaultResponse::No => false,
        },
    }
}
