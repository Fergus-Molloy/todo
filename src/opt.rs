use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Opt {
    /// Add a new task or list to your todo list
    Add {
        /// The subcommand of what you want to add can be `task` or `list`
        #[structopt(subcommand)]
        cmd: Cmd,
    },
    /// Remove completed tasks from your todo list
    Clean {
        /// Remove completed tasks in this list (defaults to active list if not given)
        list: Option<String>,
    },
    /// Mark a task as completed
    Complete {
        /// The number of the task that has been completed
        num: i32,
        /// List containing the task you want to complete (defaults to active if not given)
        list: Option<String>,
    },
    /// Get the names of all lists (current is marked with *)
    Lists,
    /// Edit the description of a task
    Edit {
        #[structopt(short, long)]
        /// The list containing the task to be altered (defaults to active list if not given)
        list: Option<String>,
        /// Number of the task you're updating
        num: i32,
        #[structopt(name = "DESC", parse(from_str))]
        /// The description to change to
        data: Vec<String>,
    },
    /// List your current tasks
    Tasks {
        /// The name of the task list to list tasks from (defaults to active list if not given)
        list: Option<String>,
        #[structopt(short, long)]
        /// Ordering of the tasks `num` for numerical order or blank for priority ordering
        order: Option<String>,
    },
    /// Remove a task or list
    Remove {
        /// Subcommand to choose what to remove can be `task` or `list`
        #[structopt(subcommand)]
        cmd: RCmd,
    },
    /// Swap the nums (can swap the order depending on print order)
    Swap {
        /// Swap nums on this list (defaults to active list if not given)
        #[structopt(short, long)]
        list: Option<String>,
        num_one: i32,
        num_two: i32,
    },
    /// Make the given list the active list
    Switch { list: String },
    /// Update nums so there are no gaps
    /// (may arbitrarily change the order)
    Update {
        /// List to update
        list: Option<String>,
    },
    /// Test command please ignore
    Test,
}

#[derive(StructOpt, Debug)]
pub enum Cmd {
    /// Add a list
    List {
        /// Name of the list to add
        list_name: String,
    },
    /// Add a task
    Task {
        #[structopt(short, long)]
        /// The priority of the given task (defaults to LOW)
        priority: Option<i32>,
        #[structopt(short, long)]
        /// The list to add the task to (defaults to active list if not given)
        list: Option<String>,
        #[structopt(name = "TASK", parse(from_str))]
        // The task to add (doesn't need to be in "")
        data: Vec<String>,
    },
}

#[derive(StructOpt, Debug)]
pub enum RCmd {
    /// Remove a list
    List {
        /// Name of the list to remove
        list_name: String,
    },
    /// Remove a task
    Task {
        /// Number of the task to be removed
        num: i32,
        /// List to remove the task from (defaults to active list if not given)
        list: Option<String>,
    },
}
