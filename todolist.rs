use std::io;

struct Todo {
    task: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        println!("1. Add a task");
        println!("2. View tasks");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                println!("Enter the task:");

                let mut task = String::new();
                io::stdin().read_line(&mut task).expect("Failed to read line");

                todos.push(Todo {
                    task: task.trim().to_string(),
                    completed: false,
                });

                println!("Task added!");
            }
            "2" => {
                println!("To-Do List:");

                for (i, todo) in todos.iter().enumerate() {
                    println!("{}: {} [{}]", i + 1, todo.task, if todo.completed { "x" } else { " " });
                }
            }
            "3" => break,
            _ => println!("Invalid choice!"),
        }
    }
}
