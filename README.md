# todo-list
A basic todo list cli program in rust
## Features
- Dynamically changing todo list depending on current directory
- Check off items and mass remove them
- Import TODO comments from source files
## Design
The program pulls a ".todo" file from the current directory or if none is found uses a global todo list speciffied by the "GLOBAL_TODO_PATH" variable
