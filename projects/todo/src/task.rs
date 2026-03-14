use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(id: usize, description: &str) -> Self {
        Self {
            id,
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.completed = true;
    }

    pub fn print(&self, index: usize) {
        if self.completed {
            println!("[x]   |   {index}   |   {}   |   {}", self.id, self.description);
        }else{
            println!("[ ]   |   {index}   |   {}   |   {}", self.id, self.description);
        }
    }
}