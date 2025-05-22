use chrono::NaiveDate;
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Eq, PartialEq)]
struct TodoItem {
    date: NaiveDate,
    task: String,
}

impl Ord for TodoItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for TodoItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("----------------------------------");
        println!("1: Display Todo Items");
        println!("2: Exit");
        println!("----------------------------------");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let todo_file_path = "src/todo.txt";
                let contents = fs::read_to_string(todo_file_path).expect("Unable to read file");

                let mut todo_items: Vec<TodoItem> = contents
                    .lines()
                    .map(|line| {
                        let parts: Vec<&str> = line.splitn(2, "  ").collect();
                        let date = NaiveDate::parse_from_str(parts[0], "%Y-%m-%d").expect("Invalid date format");
                        let task = parts[1].trim().to_string();
                        TodoItem { date, task }
                    })
                    .collect();

                todo_items.sort();

                println!("----------------------------------");
                println!("| Date       | Task                |");
                println!("----------------------------------");
                for item in todo_items {
                    println!("| {} | {} |", item.date.format("%Y-%m-%d"), item.task);
                }
                println!("----------------------------------");
            }
            "2" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 1 or 2.");
            }
        }
    }
}
