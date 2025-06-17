use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoList {
    items: Vec<TodoItem>,
    next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            next_id: 1,
        }
    }
    pub fn add(&mut self, title: String){
        let item = TodoItem {
            id: self.next_id,
            title,
            completed: false,
        };
        self.items.push(item);
        self.next_id += 1;
    }

    pub fn list(&self) -> &Vec<TodoItem> {
        &self.items
    }

    pub fn complete(&mut self, id: usize) -> Result<(), String> {
        match self.items.iter_mut().find(|i| i.id == id) {
            Some(item) => {
                item.completed = true;
                Ok(())
            }
            None => Err(format!("Todo Item with ID {} not found", id)),
        }
    }

    pub fn delete(&mut self, id: usize) -> Result<(), String> {
        let position = self.items.iter().position(|i| i.id == id);
        match position {
            Some(index) => {
                self.items.remove(index);
                Ok(())
            }
            None => Err(format!("Todo Item with ID {} not found", id)),
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self)
            .map_err(|e| format!("Failed to serialize todo list: {}", e))?;

        fs::write(filename, json)
            .map_err(|e| format!("Failed to write to file {}: {}", filename, e))?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> Result<Self, String> {
        if !Path::new(filename).exists() {
            return Ok(TodoList::new());
        }
        let contents = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;

        let todo_list: TodoList = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to deserialize todo list: {}", e))?;
        Ok(todo_list)
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let filename = "todo.json";
    let mut todo_list = TodoList::load_from_file(filename)
        .unwrap_or_else(|err| {
        eprintln!("Error loading todo list: {}", err);
        TodoList::new()
    });

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run add <title>");
                eprintln!("Title is required for adding a todo.");
                return;
            }
            let title = args[2..].join(" ");
            todo_list.add(title);
            println!("Todo added.");
        }

        "list" => {
            for todo in todo_list.list() {
                let status = if todo.completed { "[x]" } else { "[ ]" };
                println!("{} {} - {}", status, todo.id, todo.title);
            }
        }

        "complete" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run complete <id>");
                eprintln!("ID is required for completing a todo.");
                return;
            }
            let id: usize = args[2].parse().unwrap_or(0);
            match todo_list.complete(id) {
                Ok(_) => println!("Todo {} marked as complete.", id),
                Err(e) => eprintln!("{}", e),
            }
        }

        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: cargo run delete <id>");
                eprintln!("ID is required for deleting a todo.");
                return;
            }
            let id: usize = args[2].parse().unwrap_or(0);
            match todo_list.delete(id) {
                Ok(_) => println!("Todo {} deleted.", id),
                Err(e) => eprintln!("{}", e),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
        }
    }


    if let Err(e) = todo_list.save_to_file(filename) {
        eprintln!("Error saving todo list: {}", e);
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run add <title>      - Add a new todo");
    println!("  cargo run list             - List all todos");
    println!("  cargo run complete <id>    - Mark todo as complete");
    println!("  cargo run delete <id>      - Delete a todo");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo(){
        let mut list = TodoList::new();

        list.add("test".to_string());

        assert_eq!(list.list().len(), 1);
        assert_eq!(list.list()[0].title, "test");
        assert_eq!(list.list()[0].completed, false);
    }

    #[test]
    fn test_complete_todo(){
        let mut list = TodoList::new();
        
        list.add("test".to_string());
        let id = list.list()[0].id;
        let result = list.complete(id);

        assert!(result.is_ok());
        assert_eq!(list.list()[0].completed, true);
    }

    #[test]
    fn test_complete_nonexistent_todo() {
        let mut list = TodoList::new();

        let result = list.complete(999);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_delete_todo(){
        let mut list = TodoList::new();

        list.add("test delete me".to_string());
        let result = list.delete(1);

        assert!(result.is_ok());
        assert!(list.list().is_empty());
    }
}