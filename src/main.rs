use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{read_to_string, write};
mod input;
use input::get_input;

#[derive(Serialize, Deserialize)]
struct PasswordManager {
    credentials: HashMap<String, String>,
}

impl PasswordManager {
    fn new() -> Self {
        Self {
            credentials: HashMap::new(),
        }
    }

    fn add_credential(&mut self, service: String, password: String) {
        self.credentials.insert(service.clone(), password);
        println!("Credential added successfully.")
    }

    fn retrieve_credential(&self, service: &str) {
        match self.credentials.get(service) {
            Some(password) => println!("Password for {}: {}", service, password),
            None => println!("No credentials found for {}.", service),
        }
    }

    fn remove_credential(&mut self, service: &str) {
        match self.credentials.remove(service) {
            Some(_) => println!("Credential removed successfully."),
            None => println!("No credentials found for {}.", service),
        }
    }

    fn view_all_credentials(&self) {
        for (service, password) in &self.credentials {
            println!("Service: {}, Password: {}", service, password);
        }
    }

    fn save_to_file(&self, file_name: &str) {
        let serialized = serde_json::to_string(&self).unwrap();
        write(file_name, serialized).unwrap();
        println!("Credentials saved to {}", file_name);
    }

    fn load_from_file(&mut self, file_name: &str) {
        match read_to_string(file_name) {
            Ok(data) => {
                let deserialized: PasswordManager = serde_json::from_str(&data).unwrap();
                self.credentials = deserialized.credentials;
                println!("Credentials loaded successfully.");
            }
            Err(_) => println!("No existing file found"),
        }
    }
}

enum MenuOption {
    Add,
    Retrieve,
    Remove,
    RetrieveAll,
    Save,
    Exit,
}

impl MenuOption {
    fn from_input(input: &str) -> Self {
        match input {
            "1" => Self::Add,
            "2" => Self::Retrieve,
            "3" => Self::Remove,
            "4" => Self::RetrieveAll,
            "5" => Self::Save,
            "6" => Self::Exit,
            _ => panic!("Invalid menu option"),
        }
    }
}

fn main() {
    let mut manager = PasswordManager::new();
    manager.load_from_file("credentials.json");

    loop {
        println!(
            "
         _________________________________   
        | 1. Add a credential             |
        | 2. Retrieve a credential        |
        | 3. Remove a credential          |
        | 4. Retrieve all credentials     |
        | 5. Save credentials             |
        | 6. Exit                         |
         _________________________________
        "
        );

        let choice = get_input("Enter your choice: ");
        let menu_option = MenuOption::from_input(&choice);

        match menu_option {
            MenuOption::Add => {
                let service = get_input("Enter the service: ");
                let password = get_input("Enter the password: ");
                manager.add_credential(service, password);
            }
            MenuOption::Retrieve => {
                let service = get_input("Enter the service: ");
                manager.retrieve_credential(&service);
            }
            MenuOption::Remove => {
                let service = get_input("Enter the service: ");
                manager.remove_credential(&service);
            }
            MenuOption::RetrieveAll => manager.view_all_credentials(),
            MenuOption::Save => manager.save_to_file("credentials.json"),
            MenuOption::Exit => {
                manager.save_to_file("credentials.json");
                break;
            }
        }
    }
}
