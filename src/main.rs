use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{self, BufReader};
// use std::vec; Write removed from above

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    task: String,
    completed: bool,
    due_date: Option<NaiveDate>, // Adding due_date, it can be optional
}

impl Todo {
    fn new(task: String, due_date: Option<NaiveDate>) -> Self {
        Todo {
            task,
            completed: false,
            due_date,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }
}

fn main() {
    let mut todos: Vec<Todo> = load_tasks().unwrap_or_else(|_| Vec::new());

    loop {
        println!("Todo List CLI");
        println!("1: Add a new task");
        println!("2: View tasks");
        println!("3: Mark a task as completed");
        println!("4: Remove a task");
        println!("5: View tasks by date order");
        println!("6: Search tasks by keyword");
        println!("7: Save and quit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter the task:");
                let mut task = String::new();
                io::stdin()
                    .read_line(&mut task)
                    .expect("Failed to read line");
                let task = task.trim().to_string();

                println!("Enter the due date (YYYY-MM-DD) or press enter to skip:");
                let mut due_date_input = String::new();
                io::stdin()
                    .read_line(&mut due_date_input)
                    .expect("Failed to read line");
                let due_date = match due_date_input.trim() {
                    "" => None,
                    date_str => match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                        Ok(date) => Some(date),
                        Err(_) => {
                            println!("Invalid date format. Please enter in YYYY-MM-DD format.");
                            continue;
                        }
                    },
                };

                todos.push(Todo::new(task, due_date));
            }
            2 => {
                println!("Todo List:");
                for (index, todo) in todos.iter().enumerate() {
                    let status = if todo.completed { "✔" } else { " " };
                    let due = if let Some(date) = &todo.due_date {
                        date.format("%Y-%m-%d").to_string()
                    } else {
                        "No due date".to_string()
                    };
                    println!("{}: [{}] {} (Due: {})", index + 1, status, todo.task, due);
                }
            }
            3 => {
                println!("Enter the number of the task to mark as completed:");
                let mut task_num = String::new();
                io::stdin()
                    .read_line(&mut task_num)
                    .expect("Failed to read line");
                let task_num: usize = match task_num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number.");
                        continue;
                    }
                };
                if let Some(todo) = todos.get_mut(task_num - 1) {
                    todo.mark_completed();
                } else {
                    println!("Task not found.");
                }
            }
            4 => {
                println!("Enter the number of the task to remove:");
                let mut task_num = String::new();
                io::stdin()
                    .read_line(&mut task_num)
                    .expect("Failed to read line");
                let task_num: usize = match task_num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid number.");
                        continue;
                    }
                };
                if task_num > 0 && task_num <= todos.len() {
                    todos.remove(task_num - 1);
                } else {
                    println!("Task not found.");
                }
            }
            5 => {
                println!("Todo List by date:");

                // Sort the original todos vector by due date, using None as the lowest priority.
                todos.sort_by_key(|todo| todo.due_date);

                // Iterate through the sorted vector
                for (index, todo) in todos.iter().enumerate() {
                    let status = if todo.completed { "✔" } else { " " };
                    let due = if let Some(date) = &todo.due_date {
                        date.format("%Y-%m-%d").to_string()
                    } else {
                        "No due date".to_string()
                    };
                    println!("{}: [{}] {} (Due: {})", index + 1, status, todo.task, due);
                }
            }
            6 => {
                println!("Todo List, search by keyword:");
                println!("Enter keyword:");

                let mut keyword = String::new();
                io::stdin()
                    .read_line(&mut keyword)
                    .expect("Failed to read line");

                // Trim the keyword to remove any extra whitespace
                let keyword = keyword.trim().to_lowercase(); // Also convert to lowercase

                for (index, todo) in todos.iter().enumerate() {
                    if todo.task.to_lowercase().contains(&keyword) {
                        let status = if todo.completed { "✔" } else { " " };
                        let due = if let Some(date) = &todo.due_date {
                            date.format("%Y-%m-%d").to_string()
                        } else {
                            "No due date".to_string()
                        };
                        println!("{}: [{}] {} (Due: {})", index + 1, status, todo.task, due);
                    }
                }
            }
            7 => {
                save_tasks(&todos).expect("Failed to save tasks.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn load_tasks() -> Result<Vec<Todo>, std::io::Error> {
    let file = OpenOptions::new().read(true).open("todos.json").ok();

    if let Some(file) = file {
        let reader = BufReader::new(file);
        let todos = serde_json::from_reader(reader)?;
        Ok(todos)
    } else {
        Ok(Vec::new())
    }
}

fn save_tasks(todos: &[Todo]) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("todos.json")?;
    serde_json::to_writer(file, todos)?;
    Ok(())
}
