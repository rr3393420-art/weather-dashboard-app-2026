use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub completed: bool,
}

#[derive(Default)]
pub struct TodoStore {
    tasks: Vec<Todo>,
}

impl TodoStore {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(path)
            .with_context(|| format!("could not read task storage at {}", path.display()))?;
        let tasks = serde_json::from_str(&contents)
            .with_context(|| format!("invalid task data in {}", path.display()))?;
        Ok(Self { tasks })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let contents = serde_json::to_string_pretty(&self.tasks)
            .context("could not serialize tasks")?;
        fs::write(path, format!("{contents}\n"))
            .with_context(|| format!("could not save tasks to {}", path.display()))
    }

    pub fn add(&mut self, description: String) -> Result<u64> {
        let id = self
            .tasks
            .iter()
            .map(|task| task.id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
            .context("task ID limit reached")?;
        self.tasks.push(Todo {
            id,
            description,
            completed: false,
        });
        Ok(id)
    }

    pub fn mark_done(&mut self, id: u64) -> Result<()> {
        let task = self
            .tasks
            .iter_mut()
            .find(|task| task.id == id)
            .ok_or_else(|| anyhow::anyhow!("no task found with ID {id}"))?;
        task.completed = true;
        Ok(())
    }

    pub fn delete(&mut self, id: u64) -> Result<()> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or_else(|| anyhow::anyhow!("no task found with ID {id}"))?;
        self.tasks.remove(index);
        Ok(())
    }

    pub fn print_list(&self) {
        if self.tasks.is_empty() {
            println!("Your to-do list is empty. Add a task with `cargo run -- add \"Task\"`.");
            return;
        }

        println!("To-do list:");
        for task in &self.tasks {
            let status = if task.completed { "x" } else { " " };
            println!("[{status}] #{} — {}", task.id, task.description);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TodoStore;
    use std::path::Path;

    #[test]
    fn adding_tasks_assigns_increasing_ids() {
        let mut store = TodoStore::default();
        assert_eq!(store.add("First".to_owned()).unwrap(), 1);
        assert_eq!(store.add("Second".to_owned()).unwrap(), 2);
    }

    #[test]
    fn completion_and_delete_use_task_ids() {
        let mut store = TodoStore::default();
        let id = store.add("Test task".to_owned()).unwrap();
        store.mark_done(id).unwrap();
        assert!(store.tasks[0].completed);
        store.delete(id).unwrap();
        assert!(store.tasks.is_empty());
    }

    #[test]
    fn missing_storage_starts_empty() {
        let store = TodoStore::load(Path::new("this-file-should-not-exist-todo-test.json")).unwrap();
        assert!(store.tasks.is_empty());
    }
}
