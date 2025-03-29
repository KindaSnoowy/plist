use serde::{Deserialize, Serialize};
use serde_json;
use std::{env, fs, path::Path};

const FILE_NAME: &str = "tasks.json";

#[derive(Serialize, Deserialize)]
pub struct Task {
    id: u32,
    name: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TODOApp {
    tasks: Vec<Task>,
}

impl TODOApp {
    pub fn load_from_file(&mut self) {
        if Path::new(FILE_NAME).exists() {
            let file = fs::read_to_string(FILE_NAME);
            match file {
                Ok(content) => {
                    let loaded_data: Result<TODOApp, _> = serde_json::from_str(&content);
                    match loaded_data {
                        Ok(loaded_data) => {
                            self.tasks = loaded_data.tasks;
                        }
                        Err(e) => {
                            println!("Error parsing JSON: {}", e);
                            self.tasks = Vec::new();
                        }
                    }
                }
                Err(e) => {
                    println!("Error reading file: {}", e);
                }
            }
        } else {
            self.tasks = Vec::new();
        }
    }

    pub fn save_to_file(&mut self) {
        let data = serde_json::to_string_pretty(&self);
        match data {
            Ok(data) => {
                match fs::write(FILE_NAME, data) {
                    Ok(_) => (),
                    Err(e) => println!("Error writing file: {}", e),
                };
            }
            Err(e) => {
                println!("Error writing file: {}", e);
            }
        }
    }

    pub fn new(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn list(&mut self) {
        self.empty_task_save();
        for task in &self.tasks {
            println!(
                "Task ID: {}, Name: {}, Completed: {}",
                task.id,
                task.name,
                match task.completed {
                    true => "☑",
                    false => "☒",
                }
            );
        }
    }

    pub fn delete(&mut self, id: u32) {
        self.empty_task_save();
        match self.tasks.iter().position(|task| task.id == id) {
            Some(index) => {
                self.tasks.remove(index);
            }
            None => {
                println!("Task not found");
                return;
            }
        };
    }

    pub fn complete(&mut self, id: u32) {
        self.empty_task_save();
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => task.completed = true,
            None => println!("Task not found"),
        }
    }

    pub fn uncomplete(&mut self, id: u32) {
        self.empty_task_save();
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => task.completed = false,
            None => println!("Task not found"),
        }
    }

    pub fn empty_task_save(&mut self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
            self.save_to_file();
        }
    }
}

fn main() {
    let command = match env::args().nth(1) {
        Some(p) => p,
        None => {
            println!("Run with help command to see which commands are available.");
            return;
        }
    };

    let mut app = TODOApp { tasks: Vec::new() };
    app.load_from_file();

    match command.as_str() {
        "new" => {
            let args: Vec<String> = env::args().skip(2).collect();
            let task_name = args.join(" ");

            let task = Task {
                id: app.tasks.len() as u32 + 1,
                name: task_name,
                completed: false,
            };

            app.new(task);
            app.save_to_file();
            println!("Task created successfully!");
        }
        "list" => {
            app.list();
        }
        "delete" => {
            let id = match env::args().nth(2) {
                Some(id) => match id.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid task ID");
                        return;
                    }
                },
                None => {
                    println!("Task ID is required");
                    return;
                }
            };

            app.delete(id);
            println!("Task with ID {} deleted", id);
            app.save_to_file();
        }
        "complete" => {
            let id = match env::args().nth(2) {
                Some(id) => match id.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid task ID");
                        return;
                    }
                },
                None => {
                    println!("Task ID is required");
                    return;
                }
            };

            app.complete(id);
            app.save_to_file();
            println!("Task {} marked as completed!", id)
        }
        "uncomplete" => {
            let id = match env::args().nth(2) {
                Some(id) => match id.parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Invalid task ID");
                        return;
                    }
                },
                None => {
                    println!("Task ID is required");
                    return;
                }
            };

            app.uncomplete(id);
            app.save_to_file();
            println!("Task {} marked as uncompleted!", id)
        }
        "help" => {
            println!("Available commands:");
            println!("new <task_name> - Create a new task");
            println!("list - List all tasks");
            println!("delete <task_id> - Delete a task");
            println!("complete <task_id> - Mark a task as completed");
            println!("uncomplete <task_id> - Mark a task as uncompleted");
            println!("help - Display this help message");
        }
        _ => {
            println!("Invalid command, run with help command to see which commands are available.");
        }
    }
}
