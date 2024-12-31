pub fn print_help() {
    println!("Usage: todo <command> [arguments]");
    println!();
    println!("Commands:");
    println!("  help                                Show help message");
    println!("  list                                List todo items (default command)");
    println!("  add <content>                       Add item to todo list");
    println!("  remove <index | fuzzy_match>        Remove item from list");
    println!("  complete <index | fuzzy_match>      Mark item complete on list");
    println!("  uncomplete <index | fuzzy_match>    Mark item uncomplete on list");
    println!("  clean                               Remove completed items");
}
