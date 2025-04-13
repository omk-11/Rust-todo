use std::fs::{self, File};
use std::io::{self, Write};
use serde_json;
use serde::{Serialize, Deserialize};




#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name : String,
    time : String,
    status : String,
}


fn load_tasks() -> Vec<Task> {
    let file = fs::read_to_string("tasks.json").unwrap_or("[]".to_string());
    serde_json::from_str(&file).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize");
    let mut file = File::create("tasks.json").expect("Failed to create file");
    file.write_all(data.as_bytes()).expect("Failed to write data");
}

fn add_task() {
    let mut input = String::new();

        println!("Enter task description:");
        io::stdin().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        input.clear();
        println!("Enter task time (e.g. 2025-04-13 15:00):");
        io::stdin().read_line(&mut input).unwrap();
        let time = input.trim().to_string();

        input.clear();
        println!("Enter task status");
        io::stdin().read_line(&mut input).unwrap();
        let status = input.trim().to_string();

    let new_task = Task { name: name, time: time, status: status };

    let mut tasks = load_tasks();
    tasks.push(new_task);
    save_tasks(&tasks);

    println!("✅ Task added.");
}

fn delete_task() {
    let mut tasks = load_tasks();

    if tasks.is_empty() {
        println!("❌ No tasks to delete.");
        return;
    }

    println!("Enter the index of the task to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("⚠️ Please enter a valid number.");
            return;
        }
    };

    if index >= tasks.len() {
        println!("🚫 Invalid index. Please try again.");
        return;
    }

    let removed = tasks.remove(index);
    save_tasks(&tasks);

    println!("✅ Removed task: [{}] {}", removed.status, removed.name);
}


fn view_task() {
        let tasks: Vec<Task> = load_tasks();
        if tasks.is_empty() {
            println!("No tasks available.");
        } else {
            for (i, task) in tasks.iter().enumerate() {
                println!("\n📌 Task {}:", i + 1);
                println!(" Description: {}", task.name);
                println!(" Time: {}", task.time);
                println!(" Status: {}", task.status);
            }
        }
}

fn update_task() {
    let mut tasks = load_tasks();

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

    // Get the task index
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let index: usize = match input.trim().parse::<usize>() {
        Ok(num) => num - 1, // Subtract 1 for zero-indexing
        Err(_) => {
            println!("⚠️ Please enter a valid number.");
            return;
        }
    };

    if index >= tasks.len() {
        println!("🚫 Invalid task number. Please try again.");
        return;
    }

    let task_to_update = &mut tasks[index];
    
    // Ask for the update
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
            task_to_update.name = input.trim().to_string();
        }
        2 => {
            println!("Enter new time (e.g. 2025-04-13 15:00):");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            task_to_update.time = input.trim().to_string();
        }
        3 => {
            println!("Enter new status:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            task_to_update.status = input.trim().to_string();
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
    loop{

    let mut input = String::new(); 
    println!("⭐welcome to todo app⭐");
    println!("enter what you want to do ");
    println!("1. Add task");
    println!("2. delete task");
    println!("3. view task");
    println!("4. update task");
    println!("5. exit\n");

    //read the input in input variable then typecast it to integer
    input.clear();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let choice: u8 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("please enter a number"); 
            return;
        }
    };
 
    match choice{
        1 =>{
            add_task();
        }
        2 =>{
            delete_task();
        }
        3 =>{
            view_task();
        }
        4=> {
            update_task();
        }
        
        5 =>{
            println!("ok goodbye👋");
            break;
        }
        _ =>{
            println!{"please enter a valid choice"}
        }
    }
}
}