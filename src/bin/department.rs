use std::collections::hash_map::Entry;
use std::io;
use std::collections::HashMap;

fn main() {

    let mut store = HashMap::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(_) => {
                println!("No input");
                continue
            }
        };
        let input = input.to_lowercase();
        let input: Vec<&str> = input.split_whitespace().collect();

        let command = match input.get(0) {
            Some(v) => v.to_string(),
            None => {
                println!("Command not recognized, possible commands are add, remove, department, people");
                continue
            }
        };
        
        if command == "add" {
            let person = match input.get(1) {
                Some(v) => v.to_string(),
                None => {
                    println!("A user must be specified when adding (Add user to department)");
                    continue
                }
            };
            let department = match input.get(3) {
                Some(v) => v.to_string(),
                None => {
                    println!("A department must be specified when adding (Add user to department)");
                    continue
                }
            };

            match store.entry(department) {
                Entry::Vacant(e) => { e.insert(vec![person]); },
                Entry::Occupied(mut e) => { e.get_mut().push(person); }
            };
        } else if command == "remove" {
            let person = match input.get(1) {
                Some(v) => v.to_string(),
                None => {
                    println!("A user must be specified when removing (Remove user from department)");
                    continue
                }
            };
            let department = match input.get(3) {
                Some(v) => v.to_string(),
                None => {
                    println!("A department must be specified when removing (Remove user from department)");
                    continue
                }
            };

            match store.entry(department) {
                Entry::Vacant(_) => {
                    println!("Department does not exist");
                    continue;
                },
                Entry::Occupied( mut e) => {
                    let list = e.get_mut();
                    list.retain(|v| {
                        *v != person
                    })
                }
            };
            
        } else if command == "list" {
            let department = match input.get(1) {
                Some(v) => v.to_string(),
                None => {
                    println!("List requires a department (List department)");
                    continue
                }
            };
            let members = match store.get(&department) {
                Some(v) => v,
                None => {
                    println!("Department not found");
                    continue
                }
            };
            println!("{:?}", members);
            continue;
        } else if command == "people" {
            
        } else if command == "exit" {break} else {
            println!("Command not recognized, possible commands are add, remove, department, people");
            continue;
        }
        println!("{:?}", store)
    }
}