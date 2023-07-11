use std::io;
use std::collections::{HashMap, HashSet};

fn main() {
    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    let mut num_vec: Vec<u8> = vec![1, 5, 10, 2, 15, 69, 200, 69, 150, 23, 85];
    
    num_vec.sort();

    let vec_middle_index = num_vec.len() / 2;

    println!("Median value: {}", num_vec[vec_middle_index]);

    let largest_val = vector_most_common_occurence(&num_vec);

    println!("Most common occurence: {}", largest_val);

    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    const PHRASE: &str = "Hello my name is Brandon what is your name";
    let pig_latin = english_to_pig_latin(PHRASE);

    println!("{pig_latin}");

    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
    // For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    // Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Input a command (add <department> <name>, get <department>, getAll, or exit):");

        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command: String = command.trim().to_ascii_lowercase();

        if command.starts_with("add") {
            let parts: Vec<&str> = command.split(' ').collect();
            let department = parts.get(1);
            let employee = parts.get(2);

            if department.is_none() {
                println!("Missing department and employee name, please try again...");
                continue;
            } else if employee.is_none() {
                println!("Missing employee name, please try again...");
                continue;
            }

            let department = department.unwrap().to_string();
            let employee = employee.unwrap().to_string();

            let department_list = departments.entry(department.clone()).or_insert_with(Vec::new);
            department_list.push(employee.clone());

            println!("Added {} to department {}", employee, department);
        } else if command.starts_with("getall") {
            // TODO: Impl getall employees
            let mut all_employees: Vec<String> = Vec::new();
            for (_, employees) in &departments {
                all_employees.append(&mut employees.clone());
            }

            let employee_set: HashSet<String> = all_employees.into_iter().collect();
            let mut dedupped_employees: Vec<String> = employee_set.into_iter().collect();

            if dedupped_employees.len() > 0 {
                dedupped_employees.sort();

                println!("Employees: {}", dedupped_employees.join(", "));
            } else {
                println!("No employees found...");
            }
        }  else if command.starts_with("get") {
            let parts: Vec<&str> = command.split(' ').collect();
            let department = parts.get(1);

            if department.is_none() {
               println!("Missing department, please try again...");
               continue;
            }

            let department = department.unwrap().to_string();
            let employees = departments.get(&department);

            match employees {
               Some(employees) => {
                   // I know this is inefficient, sorted insert would be much better
                   let mut sorted_employees = employees.clone();
                   sorted_employees.sort();

                   println!("{}", sorted_employees.join(", "));
               },
               None => println!("No employees found in {} department", department)
            }
        }  else if command == "exit" {
            break;
        } else {
            println!("Invalid command, please try again...");
        }

        println!();
    }
}

fn vector_most_common_occurence(vec: &Vec<u8>) -> u8 {
    let mut largest_val: Option<u8> = None;
    let mut repeat_tracking: HashMap<u8, i32> = HashMap::new();
    for value in vec {
        let occurences = repeat_tracking.get(value).unwrap_or(&0);
        let new_occurences = *occurences + 1;
        repeat_tracking.insert(*value, new_occurences);

        match largest_val {
            Some(l_val) => {
                let largest_occurences = repeat_tracking.get(&l_val).unwrap_or(&0);
                if &new_occurences > largest_occurences {
                    largest_val = Some(*value);
                }
            },
            None => {
                largest_val = Some(*value);
            }
        }
    }

    return largest_val.unwrap_or(0);
}

fn english_to_pig_latin(english: &str) -> String {
    let vowels: HashSet<char> = HashSet::from(['a', 'e', 'i', 'o', 'u']);
    let mut pig_latin = String::new();

    let mut first_char = ' ';
    for c in english.trim().chars() {
        if first_char == ' ' {
            first_char = c;

            if vowels.contains(&c.to_ascii_lowercase()) {
                pig_latin.push(c);
            }
        } else {
            if &c != &' ' {
                pig_latin.push(c);
            }
        }

        if c == ' ' {
            if vowels.contains(&first_char.to_ascii_lowercase()) {
                pig_latin.push_str("-hay ");
            } else {
                pig_latin.push_str(&format!("-{}ay ", first_char.to_ascii_lowercase()));
            }

            first_char = ' ';
        }
    }

    if vowels.contains(&first_char.to_ascii_lowercase()) {
        pig_latin.push_str("-hay ");
    } else {
        pig_latin.push_str(&format!("-{}ay ", first_char.to_ascii_lowercase()));
    }

    return pig_latin;
}
