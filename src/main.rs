use serde::{Serialize, Deserialize};
use std::io::{self, Write};
use std::fs::{File, OpenOptions};
use std::io::BufWriter;

#[derive(Serialize, Deserialize, Debug)]
struct PasswordEntry {
    website: String,
    username: String,
    password: String,
}

fn main() {
    let mut entries: Vec<PasswordEntry> = load_entries();

    loop {
        println!("\nPassword Manager");
        println!("1. Add password");
        println!("2. List passwords");
        println!("3. Quit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let entry = create_entry();
                entries.push(entry);
                save_entries(&entries);
                println!("Password added!");
            }
            "2" => {
                if entries.is_empty() {
                    println!("No passwords stored.");
                } else {
                    for (i, entry) in entries.iter().enumerate() {
                        println!(
                            "{}. Website: {}, Username: {}, Password: {}",
                            i + 1,
                            entry.website,
                            entry.username,
                            entry.password
                        );
                    }
                }
            }
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

fn save_entries(entries: &Vec<PasswordEntry>) {
    let file = File::create("passwords.json").expect("Unable to create file");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, entries).expect("Unable to write data");
}

fn load_entries() -> Vec<PasswordEntry> {
    let file = OpenOptions::new().read(true).open("passwords.json");
    if let Ok(file) = file {
        serde_json::from_reader(file).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn create_entry() -> PasswordEntry {
    let website = prompt("Enter website: ");
    let username = prompt("Enter username: ");
    let password = prompt("Enter password: ");
    PasswordEntry { website, username, password }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}