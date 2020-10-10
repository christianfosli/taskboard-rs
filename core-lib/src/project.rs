#[deny(missing_docs)]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Projects are used to group tasks
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Project {
    /// Unique identifier
    pub id: Uuid,
    /// Friendly name
    pub name: String,
    /// The total number of tasks created for this project
    pub task_conter: usize,
}

impl Project {
    /// Creates a new project
    pub fn new(name: &str) -> Self {
        Project {
            id: Uuid::new_v4(),
            name: String::from(name),
            task_conter: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_set_task_counter_to_zero() {
        assert_eq!(Project::new("learn rust").task_conter, 0);
    }

    #[test]
    fn new_should_use_unique_ids() {
        assert_ne!(Project::new("").id, Project::new("").id);
    }
}
