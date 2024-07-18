use clap::{Parser, Subcommand, ValueEnum};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Show {
        list_name: Option<String>,
        #[clap(value_enum)]
        filter: Option<Filter>,
    },
    Add {
        list_name: String,
        item: String,
    },
    Complete {
        list_name: String,
        item_number: usize,
    },
    Incomplete {
        list_name: String,
        item_number: usize,
    },
    Remove {
        list_name: Option<String>,
        item_number: Option<usize>,
    },
}

#[derive(ValueEnum, Clone)]
enum Filter {
    All,
    Completed,
    Incomplete,
}

#[derive(Serialize, Deserialize, Default)]
struct TodoList {
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add(&mut self, item: String) {
        self.tasks.push(Task { description: item, completed: false });
    }

    fn complete(&mut self, item_number: usize) {
        if let Some(task) = self.tasks.get_mut(item_number) {
            task.completed = true;
        }
    }

    fn incomplete(&mut self, item_number: usize) {
        if let Some(task) = self.tasks.get_mut(item_number) {
            task.completed = false;
        }
    }

    fn remove(&mut self, item_number: usize) {
        if item_number < self.tasks.len() {
            self.tasks.remove(item_number);
        }
    }

    fn show(&self, filter: Option<Filter>) {
        for (i, task) in self.tasks.iter().enumerate() {
            match filter {
                Some(Filter::Completed) if !task.completed => continue,
                Some(Filter::Incomplete) if task.completed => continue,
                _ => (),
            }
            println!("{}: [{}] {}", i, if task.completed { "x" } else { " " }, task.description);
        }
    }
}

fn load_lists() -> HashMap<String, TodoList> {
    let path = "todo_lists.json";
    if Path::new(path).exists() {
        let data = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data).expect("Unable to parse JSON")
    } else {
        HashMap::new()
    }
}

fn save_lists(lists: &HashMap<String, TodoList>) {
    let data = serde_json::to_string_pretty(lists).expect("Unable to serialize");
    fs::write("todo_lists.json", data).expect("Unable to write file");
}

fn main() {
    let cli = Cli::parse();
    let mut lists = load_lists();

    match cli.command {
        Commands::Show { list_name, filter } => {
            if let Some(name) = list_name {
                if let Some(list) = lists.get(&name) {
                    list.show(filter);
                } else {
                    println!("List not found");
                }
            } else {
                for (name, list) in &lists {
                    println!("List: {}", name);
                    list.show(filter.clone());
                }
            }
        }
        Commands::Add { list_name, item } => {
            lists.entry(list_name.clone()).or_insert_with(TodoList::new).add(item);
            save_lists(&lists);
        }
        Commands::Complete { list_name, item_number } => {
            if let Some(list) = lists.get_mut(&list_name) {
                list.complete(item_number);
                save_lists(&lists);
            } else {
                println!("List not found");
            }
        }
        Commands::Incomplete { list_name, item_number } => {
            if let Some(list) = lists.get_mut(&list_name) {
                list.incomplete(item_number);
                save_lists(&lists);
            } else {
                println!("List not found");
            }
        }
        Commands::Remove { list_name, item_number } => {
            match (list_name, item_number) {
                (Some(name), Some(number)) => {
                    if let Some(list) = lists.get_mut(&name) {
                        list.remove(number);
                        save_lists(&lists);
                    } else {
                        println!("List not found");
                    }
                }
                (Some(name), None) => {
                    lists.remove(&name);
                    save_lists(&lists);
                }
                (None, None) => {
                    lists.clear();
                    save_lists(&lists);
                }
                _ => println!("Invalid command"),
            }
        }
    }
}
