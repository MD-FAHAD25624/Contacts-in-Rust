use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

// the file where contacts are saved between sessions
const SAVE_FILE: &str = "contacts.txt";
const DELIM: char = '|';

// -------------------------------------------------------
// Saves all contacts to contacts.txt
// Each line holds one contact: name|number
// -------------------------------------------------------
fn save_contacts(contacts: &HashMap<String, String>) {
    let mut lines: Vec<String> = Vec::new();

    for (name, number) in contacts {
        // Replace any | in the data with [PIPE] so the
        // delimiter never appears inside a field value
        let safe_name = name.replace(DELIM, "[PIPE]");
        let safe_number = number.replace(DELIM, "[PIPE]");
        lines.push(format!("{}|{}", safe_name, safe_number));
    }

    let content = lines.join("\n");

    match fs::write(SAVE_FILE, content) {
        Ok(_) => {}
        Err(e) => {
            println!("Warning: Could not save contacts: {}", e);
        }
    }
}

// -------------------------------------------------------
// Loads contacts from contacts.txt at startup
// Returns an empty HashMap if the file does not exist yet
// -------------------------------------------------------
fn load_contacts() -> HashMap<String, String> {
    let mut contacts: HashMap<String, String> = HashMap::new();

    if !Path::new(SAVE_FILE).exists() {
        return contacts;
    }

    let content = match fs::read_to_string(SAVE_FILE) {
        Ok(c) => c,
        Err(e) => {
            println!("Warning: Could not load contacts: {}", e);
            return contacts;
        }
    };

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, DELIM).collect();
        if parts.len() == 2 {
            let name = parts[0].replace("[PIPE]", "|");
            let number = parts[1].replace("[PIPE]", "|");
            contacts.insert(name, number);
        } else {
            println!("Warning: skipping malformed line: {}", line);
        }
    }

    contacts
}

// -------------------------------------------------------
// OPTION 1: Add a contact
// -------------------------------------------------------
fn add_contact(contacts: &mut HashMap<String, String>) {
    show_all_contacts(contacts);
    println!("Enter the contact's name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    let name = capitalise_first(name.trim());

    if contacts.contains_key(&name) {
        println!(
            "Warning: Contacts already contains {}. Overwrite? (y/n)",
            name
        );
        let mut answer = String::new();
        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read input");
        match answer.trim().to_lowercase().as_str() {
            "y" => {}
            _ => {
                println!("Addition cancelled.");
                return;
            }
        }
    }

    println!("Enter the phone number:");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");
    let number = number.trim().to_string();

    contacts.insert(name.clone(), number);
    save_contacts(contacts);

    println!("Contact '{}' saved successfully!", name);
}

// -------------------------------------------------------
// OPTION 2: Look up a contact
// -------------------------------------------------------
fn search_contact(contacts: &HashMap<String, String>) {
    println!("Enter the name to search:");
    let mut search = String::new();
    io::stdin()
        .read_line(&mut search)
        .expect("Failed to read input");
    let search = capitalise_first(search.trim());

    match contacts.get(&search) {
        Some(number) => println!("Phone number: {}", number),
        None => println!("No contact found for '{}'.", search),
    }
}

// -------------------------------------------------------
// OPTION 3: Show all contacts (sorted, numbered)
// -------------------------------------------------------
fn show_all_contacts(contacts: &HashMap<String, String>) {
    if contacts.is_empty() {
        println!("No contacts saved yet.");
        return;
    }

    println!("\n--- All Contacts ---");
    let mut sorted: Vec<_> = contacts.iter().collect();
    sorted.sort();

    for (i, (name, number)) in sorted.iter().enumerate() {
        println!("{}. {} --> {}", i + 1, name, number);
    }
}

// -------------------------------------------------------
// OPTION 4: Delete a contact (with confirmation)
// -------------------------------------------------------
fn delete_contact(contacts: &mut HashMap<String, String>) {
    show_all_contacts(contacts);
    println!("Enter the name of the contact to delete:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    let name = capitalise_first(name.trim());

    if !contacts.contains_key(&name) {
        println!("No contact found for '{}'.", name);
        return;
    }

    println!("Are you sure you want to delete '{}'? (y/n)", name);
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input");

    match answer.trim().to_lowercase().as_str() {
        "y" => {
            contacts.remove(&name);
            save_contacts(contacts);
            println!("Contact '{}' deleted.", name);
        }
        _ => println!("Deletion cancelled."),
    }
}

// -------------------------------------------------------
// Main menu loop
// -------------------------------------------------------
fn main() {
    let mut contacts: HashMap<String, String> = load_contacts();

    loop {
        println!("\n===== CONTACTS APP =====");
        println!("1. Add a contact");
        println!("2. Look up a contact");
        println!("3. Show all contacts");
        println!("4. Delete a contact");
        println!("5. Exit");
        println!("========================");
        println!("Enter your choice:");

        let mut choice_input = String::new();
        io::stdin()
            .read_line(&mut choice_input)
            .expect("Failed to read input");

        let choice: usize = match choice_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number between 1 and 5.");
                continue;
            }
        };

        match choice {
            1 => add_contact(&mut contacts),
            2 => search_contact(&contacts),
            3 => show_all_contacts(&contacts),
            4 => delete_contact(&mut contacts),
            5 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Please choose 1 to 5."),
        }
    }
}
fn capitalise_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().to_string() + &chars.as_str(),
    }
}
