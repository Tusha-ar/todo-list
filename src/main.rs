use std::{
    env, fmt::Display, fs::{self, OpenOptions}, io::{BufRead, BufReader, Write}, path::Path, process
};

const FILE_NAME: &str = "todo.txt";
struct Todo {
    description: String,
    completed: bool,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "description: {}, completed: {}",
            self.description, self.completed
        )
    }
}

impl Todo {
    fn new(description: String) -> Todo {
        let new_todo = Todo {
            completed: false,
            description,
        };
        if Path::new(FILE_NAME).exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(FILE_NAME)
                .unwrap();
            let content = format!("{}\n", new_todo);
            file.write_all(content.as_bytes()).unwrap();
        } else {
            let content = format!("{}\n", new_todo);
            let mut file = fs::File::create(FILE_NAME).unwrap();
            file.write(content.as_bytes()).unwrap();
            
        }
        new_todo
    }

    fn drop(index: u16) {
        let mut todos = read_todos();
        if usize::from(index) < todos.len() {
            todos.remove(index.into());
            let success_string = format!("Dropped task {} sucessfully", index + 1);
            let err_string = format!("Error while dropping the task at {}", index + 1);
            open_file_and_write(todos, success_string, err_string);
        } else {
            eprintln!("Enter a valid index");
            process::exit(0)
        }
    }

    fn complete(index: u16) {
        let mut todos = read_todos();

        if usize::from(index) < todos.len() {
            todos[usize::from(index)].completed = true;
            let success_string = format!("task at index {} marked as complete", index + 1);
            let err_string = format!("Error while dropping the task at {}", index + 1);
            open_file_and_write(todos, success_string, err_string);
        } else {
            eprintln!("Enter a valid index");
            process::exit(0)
        }
    }

    fn edit(index: u16, new_description: String) {
        let mut todos = read_todos();

        if usize::from(index) < todos.len() {
            todos[usize::from(index)].description = new_description;
            let success_string = format!("Editied todo at index {} sucessfully", index + 1);
            let err_string = format!("Error while editing todo at index {}", index + 1);
            open_file_and_write(todos, success_string, err_string);
        } else {
            eprintln!("Enter a valid index");
            process::exit(0);
        }
    }

    fn list() {
        let todos = read_todos();
        println!("Todos:");
        for (index, todo) in todos.iter().enumerate() {
            let emoji = if todo.completed {"✅"} else {"❌"};
            println!("{}: {} {}", index + 1, todo.description, emoji);
        }
    }
}

fn open_file_and_write(todos: Vec<Todo>, success_string: String, err_string: String) {
    let mut file = OpenOptions::new().write(true).truncate(true).open(FILE_NAME).unwrap();
    for todo in todos {
        let content = format!("{}\n", todo);
        if let Err(_) = file.write(content.as_bytes()) {
                eprintln!("{}", err_string);
                process::exit(0);
        };
    }
    println!("{}", success_string);
}

fn read_todos() -> Vec<Todo> {
    let mut todos: Vec<Todo> = Vec::new();
    if Path::new(FILE_NAME).exists() {
        let file = fs::File::open(FILE_NAME).expect("Unable to open file");

        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    let todo: Vec<&str> = l.split(", ").collect();
                    if todo.len() == 2 {
                        let description = todo[0].replace("description: ", "");
                        let completed = todo[1].replace("completed: ", "").parse::<bool>().unwrap();
                        todos.push(Todo {
                            description,
                            completed,
                        });
                    }
                }
                Err(_) => {
                    eprintln!("Error while parsing the content in file");
                    process::exit(0);
                }
            }
        }
    }
    todos
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Invalid Input");
        process::exit(0);
    } else {
        if args[1].contains("-h") | args[1].contains("-help") {
            println!("-add index : Add a new task");
            println!("");
            println!("-drop index : Drop an eisting task");
            println!("");
            println!("-complete index : Mark task as complete");
            println!("");
            println!("-edit index new_task : Edit an existing task");
            println!("");
            println!("-list : List all tasks");
        } else if args[1].contains("-add") {
            if args.len() < 3 {
                eprintln!("Please provide a task to enter");
                process::exit(0);
            } else {
                let task = Todo::new(args[2..].join(" ").to_string());
                println!("Todo: {} added sucessfully", task);
            }
        } else if args[1].contains("-drop") {
            if args.len() < 3 {
                eprintln!("Please provide a task to enter");
                process::exit(0);
            } else {
                let index = args[2].parse::<u16>().expect("Invalid Index") - 1;
                Todo::drop(index);
            }
        } else if args[1].contains("-complete") {
            if args.len() < 3 {
                eprintln!("Please provide a task to be marked as complete");
                process::exit(0);
            } else {
                let index = args[2].parse::<u16>().expect("Invalid Index") - 1;
                Todo::complete(index);
            }
        } else if args[1].contains("-edit") {
            if args.len() < 3 {
                eprintln!("Please provide a task index to be edited");
                process::exit(0);
            } else {
                if args.len() < 4 {
                    eprintln!("Please provide a new task to replace the older one");
                    process::exit(0);
                } else {
                    let index = args[2].parse::<u16>().expect("Invalid Index") - 1;
                    Todo::edit(index, args[3].clone());
                }
            }
        } else if args[1].contains("-list") {
            Todo::list();
        }
    }
}
