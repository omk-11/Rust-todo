use std::fs::{self, File};
use std::io::{self, Write};
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name : String,   // The name/description of the task
    time : String,   // The time when the task is set for
    status : String, // The current status of the task (e.g., "completed", "pending")
}

// Load the tasks from the file, return an empty Vec if the file doesn't exist or is empty
fn load_tasks() -> Vec<Task> {
    let file = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    serde_json::from_str(&file).unwrap_or_else(|_| vec![])
}

// Save the tasks to the file in pretty format
fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize");
    let mut file = File::create("tasks.json").expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write data");
}

// Function to add a new task to the list
fn add_task() {
    let mut input = String::new();

    println!("Enter task description:");
    io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();  // Read task description

    input.clear();
    println!("Enter task time (e.g. 2025-04-13 15:00):");
    io::stdin().read_line(&mut input).unwrap();
    let time = input.trim().to_string();  // Read task time

    input.clear();
    println!("Enter task status:");
    io::stdin().read_line(&mut input).unwrap();
    let status = input.trim().to_string();  // Read task status

    let new_task = Task { name, time, status };  // Create new task

    let mut tasks = load_tasks();  // Load existing tasks
    tasks.push(new_task);  // Add the new task
    save_tasks(&tasks);  // Save updated tasks to the file

    println!("✅ Task added.");
}

// Function to delete a task by its index
fn delete_task() {
    let mut tasks = load_tasks();  // Load tasks

    if tasks.is_empty() {
        println!("❌ No tasks to delete.");
        return;
    }

    println!("Enter the index of the task to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let index: usize = match input.trim().parse() {
        Ok(num) => num,  // Convert input to index
        Err(_) => {
            println!("⚠️ Please enter a valid number.");
            return;
        }
    };

    if index >= tasks.len() {
        println!("🚫 Invalid index. Please try again.");
        return;
    }

    let removed = tasks.remove(index);  // Remove task from the list
    save_tasks(&tasks);  // Save updated tasks to the file

    println!("✅ Removed task: [{}] {}", removed.status, removed.name);  // Show removed task info
}

// Function to view all the tasks
fn view_task() {
    let tasks: Vec<Task> = load_tasks();  // Load tasks

    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        // Display all tasks
        for (i, task) in tasks.iter().enumerate() {
            println!("\n📌 Task {}:", i + 1);
            println!(" Description: {}", task.name);
            println!(" Time: {}", task.time);
            println!(" Status: {}", task.status);
        }
    }
}

// Function to update an existing task
fn update_task() {
    let mut tasks = load_tasks();  // Load tasks

    if tasks.is_empty() {
        println!("❌ No tasks available to update.");
        return;
    }

    // Display tasks to the user
    println!("Which task would you like to update? (Enter the task number):");
    for (i, task) in tasks.iter().enumerate() {
        println!("\n📌 Task {}:", i + 1);
        println!(" Description: {}", task.name);
        println!(" Time: {}", task.time);
        println!(" Status: {}", task.status);
    }

    // Get the task index from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) => num - 1,  // Subtract 1 for zero-indexing
        Err(_) => {
            println!("⚠️ Please enter a valid number.");
            return;
        }
    };

    if index >= tasks.len() {
        println!("🚫 Invalid task number. Please try again.");
        return;
    }

    let task_to_update = &mut tasks[index];  // Get the task to update

    // Ask the user which field they want to update
    println!("What would you like to update?");
    println!("1. Update Description");
    println!("2. Update Time");
    println!("3. Update Status");
    println!("4. Cancel");

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let choice: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("⚠️ Invalid option.");
            return;
        }
    };

    // Modify the chosen field
    match choice {
        1 => {
            println!("Enter new description:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            task_to_update.name = input.trim().to_string();  // Update description
        }
        2 => {
            println!("Enter new time (e.g. 2025-04-13 15:00):");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            task_to_update.time = input.trim().to_string();  // Update time
        }
        3 => {
            println!("Enter new status:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            task_to_update.status = input.trim().to_string();  // Update status
        }
        4 => {
            println!("No changes made.");
            return;
        }
        _ => {
            println!("⚠️ Invalid option.");
            return;
        }
    }

    // Save updated tasks back to the file
    save_tasks(&tasks);

    println!("✅ Task updated.");
}

fn main() {
    loop {
        let mut input = String::new(); 
        println!("⭐ Welcome to the Todo app ⭐");
        println!("What would you like to do?");
        println!("1. Add task");
        println!("2. Delete task");
        println!("3. View tasks");
        println!("4. Update task");
        println!("5. Exit\n");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number.");
                continue;
            }
        };

        // Match the user's choice and call the respective functions
        match choice {
            1 => add_task(),
            2 => delete_task(),
            3 => view_task(),
            4 => update_task(),
            5 => {
                println!("Goodbye! 👋");
                break;
            }
            _ => {
                println!("⚠️ Invalid choice. Please try again.");
            }
        }
    }
}
