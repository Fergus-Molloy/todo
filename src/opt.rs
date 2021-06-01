use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Opt {
    /// Add a new task to your todo list
    Add {
        #[structopt(subcommand)]
        cmd: Option<Cmd>,
        #[structopt(short, long)]
        /// The priority of the given tasks (defaults to LOW)
        priority: Option<i32>,
        #[structopt(short, long)]
        /// The list to add the task to (defaults to active list if not given)
        list: Option<String>,
        #[structopt(name = "TASK", parse(from_str))]
        // The task to add (doesn't need to be in "")
        data: Vec<String>,
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
        list: Option<String>,
    },
    /// Get the name of the current list
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
        order: Option<String>,
    },
    /// remove a task with the given num (can remove lists by name in list mode)
    Remove {
        #[structopt(subcommand)]
        list: Option<Cmd>,
        #[structopt(name = "VALUE")]
        /// name of list or num of task
        value: Option<i32>,
    },
    /// Swap the nums (can swap the order depending on print order)
    Swap {
        /// Swap nums on this list
        #[structopt(short, long)]
        list: Option<String>,
        num_one: i32,
        num_two: i32,
    },
    /// Make the given list the active list
    Switch { list: String },
    /// Remove a single item or list
    /// Update nums so there are no gaps
    /// (may arbitrarily change the order)
    Update {
        /// list to update
        list: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
pub enum Cmd {
    List { list_name: String },
}
