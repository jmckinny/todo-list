pub fn print_help() {
    println!("Usage: todo <command> [arguments]");
    println!();
    println!("Commands:");
    println!("  help                                Show help message");
    println!("  list                                List todo items (default command)");
    println!("  add <content>                       Add item to todo list (whitespace insensitve");
    println!("  adds <content> <content> ...        Add several items to todo list (whitespace seperated)");
    println!("  log <content>                       Add completed item to todo list (whitespace insensitve");
    println!("  logs <content> <content> ...        Add several completed items to todo list (whitespace seperated)");
    println!("  remove <index|fuzzy_search>         Remove item from list");
    println!("  complete <index|fuzzy_search>       Mark item complete on list");
    println!("  uncomplete <index|fuzzy_search>     Mark item uncomplete on list");
    println!("  clean                               Remove completed items");
}
