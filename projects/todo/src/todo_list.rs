use std::fs;
use crate::task::Task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    // pub fn new() -> Self {
    //     Self {
    //         tasks: Vec::new(),
    //     }
    // }

    fn generate_id(&self) -> usize {
        if self.tasks.is_empty() {
            return 1;
        }else if self.tasks.len() == 1{
            return 2;
        }else{
            let length = self.tasks.len();

            let last_task = &self.tasks[length - 1];
            let last_but_one_task = &self.tasks[length -2];
    
            return last_task.id + last_but_one_task.id;
        }
    }

    fn save(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        let _ = fs::write("todos.json", json);
    }

    pub fn load() -> Self {
        let data = fs::read_to_string("todos.json");

        match data {
            Ok(content) => serde_json::from_str(&content).unwrap(),
            Err(_) => TodoList { tasks: vec![] }
        }
    }

    pub fn add_task(&mut self, description: &str) {
        let task = Task::new(self.generate_id(), description);
        self.tasks.push(task);
        self.save();
    }

    pub fn mark_done(&mut self, index: usize) {
        if index == 0 || index > self.tasks.len() {
            println!("This index ({index}) is not valid");
            return;
        }

        let task_to_update: &mut Task = &mut self.tasks[index-1];

        task_to_update.mark_done();

        self.save();
    }

    pub fn list_tasks(&self) {
        let mut index = 0;
        for task in &self.tasks {

            index += 1;

            task.print(index);
        }
    }

    // pub fn remove_task(&mut self, index: usize) {
    //     self.tasks.remove(index);
    // }
}