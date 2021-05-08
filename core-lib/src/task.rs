#![deny(missing_docs)]
use serde::{Deserialize, Serialize};

/// A task represents a single job.
/// Usage example:
/// ```
/// use taskboard_core_lib::Task;
/// let task_1 = Task::new(1, "Learn rust");
/// let task_2 = Task::new(2, "Learn K8S");
/// assert_ne!(task_1, task_2);
/// ```
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Task {
    /// A number used for identifying tasks within a project
    pub number: usize,
    /// The task's title
    pub title: String,
    /// The task's current status
    pub status: Status,
    /// Remaning work in number of hours, or None if not specified
    pub remaining_work: Option<u8>,
}

/// A wrapper type with a list of tasks associated to a project
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProjectTasks {
    /// Name of the related project
    pub project_name: String,
    /// All tasks associated with that project
    pub tasks: Vec<Task>,
}

/// A tasks status
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Status {
    /// Not doing
    Todo,
    /// Currently doing
    Doing,
    /// Completed
    Done,
}

impl Task {
    /// Creates a new Task with status Todo
    pub fn new(number: usize, title: &str) -> Self {
        Task {
            number,
            title: String::from(title),
            status: Status::Todo,
            remaining_work: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_set_status_to_todo() {
        assert_eq!(Task::new(1, "").status, Status::Todo);
    }

    #[test]
    fn new_should_not_set_remaining_work() {
        assert_eq!(Task::new(1, "").remaining_work, None);
    }
}
