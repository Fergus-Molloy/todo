# todo
A todo list tracker written in rust with support for multiple lists

# Usage
```bash
$ todo
todo 0.1.0
simple command-line todo list

USAGE:
    todo [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add         Add a new task to your todo list
    clean       Remove completed tasks from your todo list
    complete    Mark a task as completed
    current     Get the name of the current list
    edit        Edit the description of a task
    help        Prints this message or the help of the given subcommand(s)
    list        List your current tasks
    remove      remove a task with the given num (can remove lists by name in list mode)
    swap        Swap the nums (can swap the order depending on print order)
    switch      Make the given list the active list
    update      Remove a single item or list Update nums so there are no gaps (may arbitrarily change the order)
```

# Notes
Uses an sqlite db located at `~/.todo.db` hopefully

# Todo

- [x] Automatically create database if it doesn't exist
- [ ] clean up code in database.rs (pls don't look it's terrifying)
- [ ] deal with all the different possible unexpected outcomes (silly users typing things in wrong)
