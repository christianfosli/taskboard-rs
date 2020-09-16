#[deny(missing_docs)]
use serde::{Deserialize, Serialize};

/// A task
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Task {
    /// The task's title
    pub title: String,
    /// The task's current status
    pub status: Status,
    /// Remaning work in number of hours, or None if not specified
    pub remaining_work: Option<u8>,
}

/// A wrapper type with a list of tasks associated to a project
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectTasks {
    /// Id of the project
    pub project_id: String,
    /// All tasks associated with the project
    pub tasks: Vec<Task>,
}

impl Task {
    /// Creates a new Task with status Todo
    pub fn new(title: &str) -> Task {
        Task {
            title: String::from(title),
            status: Status::Todo,
            remaining_work: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_set_status_to_todo() {
        assert_eq!(Task::new("").status, Status::Todo);
    }

    #[test]
    fn new_should_not_set_remaining_work() {
        assert_eq!(Task::new("").remaining_work, None);
    }

    #[test]
    fn tasks_with_the_same_values_should_be_equal() {
        let left = Task::new("test-title");
        let right = Task::new("test-title");

        assert_eq!(left, right);
    }
}
