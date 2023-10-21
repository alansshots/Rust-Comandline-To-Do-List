// extern crate chrono;
use chrono::{NaiveDate, Datelike};
use std::io;

fn main() {
    let mut to_do_list: Vec<String> = Vec::new();

    println!("****** Yo, Let's create a small To-Do List with your uni tasks!******");
    println!("****** Before we start please enter your name: ******");
    let mut input_name:String = String::new();
    io::stdin().read_line(&mut input_name).expect("Failed to readline please try again using only characters(A-Z/a-z)");
    print!("Awesome name, {}", input_name);

    println!("****** Date:******");
    let mut input_date:String = String::new();
    io::stdin().read_line(&mut input_date).expect("Failed to readline please try again in the following format: DD/MM/YYYY ");
    let input_date = input_date.trim();
    match NaiveDate::parse_from_str(&input_date, "%d/%m/%Y") {
        Ok(parsed_date) => {
            // Check if the parsed date is a weekend
            if is_weekend(parsed_date) {
                println!("{} Reeelax, who works on weekends? Anyway let's add some tasks. ", parsed_date);
            } else {
                println!("{} Looks like a work day to me. Let's add some tasks. ", parsed_date);
            }
        }
        Err(err) => {
            eprintln!("Failed to parse the date: {}", err);
        }
    }

    println!("****** Time to add some tasks!******");
    println!("Enter (1) to add a New Task. ");
    println!("Enter (2) to show All Tasks. ");
    println!("Enter (3) to Quit.");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    while choice != "3" {
    match choice.trim() {
        "1" => {
            println!("Enter a task description:");
                let mut task_description = String::new();
                io::stdin()
                    .read_line(&mut task_description)
                    .expect("Failed to read line.");
                to_do_list.push(task_description.trim().to_string());
        }
        "2" => {
            println!("Showing All Tasks:");
            if to_do_list.is_empty() {
                println!("Your to-do list is empty.");
                println!("Enter (1) to add a New Task. ");
                println!("Enter (2) to show All Tasks. ");
                println!("Enter (3) to Quit.");
            } else {
                println!("****** To-Do List ******");
                for (index, task) in to_do_list.iter().enumerate() {
                    println!("{}. {}", index + 1, task);
                }
                println!("________________________");
                println!("Enter (1) to add a New Task. ");
                println!("Enter (2) to show All Tasks. ");
                println!("Enter (3) to Quit.");
            }
        }
        "3" => {
            println!("Goodbye.");
            break;
        }
        _ => println!("Invalid choice. Please select 1, 2 or 3.")
    }
        let mut next_choice = String::new();
        io::stdin().read_line(&mut next_choice).expect("Failed to read line");
        choice = next_choice.trim().to_string(); 
  }

}

fn is_weekend(date: NaiveDate) -> bool {
    let day_of_week = date.weekday().number_from_monday();
    return day_of_week == 6 || day_of_week == 7;
}