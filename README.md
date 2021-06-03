# todo
A todo list tracker written in rust with support for multiple lists

# Usage
```
$ todo
todo 0.1.0

USAGE:
    todo <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add         Add a new task or list to your todo list
    clean       Remove completed tasks from your todo list
    complete    Mark a task as completed
    edit        Edit the description of a task
    help        Prints this message or the help of the given subcommand(s)
    lists       Get the names of all lists (current is marked with *)
    remove      Remove a task or list
    swap        Swap the nums (can swap the order depending on print order)
    switch      Make the given list the active list
    tasks       List your current tasks
    test        Test command please ignore
    update      Update nums so there are no gaps (may arbitrarily change the order)
```

# Notes
Uses an sqlite db located at `~/.todo.db`

# Todo

- [x] Automatically create database if it doesn't exist
- [x] clean up code in database.rs (pls don't look it's terrifying)
- [ ] deal with all the different possible unexpected outcomes (silly users typing things in wrong)
- [ ] add a logger to tidy up some of the outputs
