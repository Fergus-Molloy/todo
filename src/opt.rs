use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Opt {
    /// Add a new task to your todo list
    Add {
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
    },
    /// List your current tasks
    List {
        /// The name of the task list to list tasks from (defaults to active list if not given)
        list: Option<String>,
    },
    /// Remove a single item or list
    Remove {
        #[structopt(short, long)]
        /// Remove a list instead
        list_mode: bool,
        #[structopt(name = "VALUE")]
        /// name of list or num of task
        value: String,
    },
    /// Update nums so they're all in order :)
    Update,
    /// Edit the description of a task
    Edit,
    /// Swap the nums (can swap the order depending on print order)
    Swap,
    /// Get the name of the current list
    Current,
}
