use std::collections::HashMap;
use std::io::{stdin};

/*
    Homework from chapter 8

    Using a hash map and vectors, create a text interface to allow a user to add employee names to a
    department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
    let the user retrieve a list of all people in a department or all people in the company by
    department, sorted alphabetically.
 */
fn main() {
    let mut department_map = HashMap::new();

    loop {
        println!("----------------------------------------");
        println!("Valid commands");
        println!("- Add {{employee}} to {{department}}");
        println!("- Retrieve list for {{department}}");
        println!("- Retrieve all");
        println!("- Quit");

        let mut input = String::new();
        // Read line + Early exit if line couldn't be read
        if let Err(e) = stdin().read_line(&mut input) {
            println!("Could not read input from stdin: {}", e);
            break;
        }

        let input = input.trim().to_lowercase();
        let words = input.split_whitespace().collect::<Vec<&str>>();

        match words[..] {
            ["add", employee, "to", department] => {
                let employees = department_map
                    .entry(String::from(department))
                    .or_insert(Vec::new());

                employees.push(String::from(employee));
                // Sort employees when we add so we don't need to when we print
                employees.sort();

                println!("Added {employee} to {department}");
            },
            ["retrieve", "list", "for", department] => {
                print_department(&department_map, department);
            },
            ["retrieve", "all"] => {
                // Get all the departments in the `department_map` hashmap
                let mut departments = department_map.keys().collect::<Vec<&String>>();
                // Sort departments alphabetically
                departments.sort();

                for department in departments {
                    print_department(&department_map, department);
                }
            },
            ["quit"] => {
                println!("Goodbye");
                break;
            },
            _ => {
                println!("Invalid command");
            }
        }
    }
}

fn print_department(department_map: &HashMap<String, Vec<String>>, department: &str) {
    let employees = department_map.get(department);
    match employees {
        Some(employees) => {
            println!("Employees in {department} department");
            print_employees(employees);
        },
        None => {
            println!("No employees found for {department}");
        }
    }
}

fn print_employees(employees: &Vec<String>) {
    for employee in employees {
        println!("- {employee}");
    }
}