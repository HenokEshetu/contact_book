pub mod contact;
pub mod operation;

use std::collections::HashMap;
use contact::{Contact, add_contact, delete_contact, edit_contact, print_contacts, load_contacts};
use operation::{Operation, do_operation};
use crate::contact::save_contacts;

type PhoneNumber = String;
type Contacts = HashMap<String, Contact>;

fn main() {
    let mut contacts = load_contacts().expect("Error loading contacts.");
    // let mut contacts = Contacts::new();
    let mut quit = false;

    while !quit {
        println!("================================================");
        println!("=========== Welcome to Contact Book  ===========");
        println!("================================================");
        println!("================================================");
        println!("=========== (1) Print all contact    ===========");
        println!("=========== (2) Add contact          ===========");
        println!("=========== (3) Edit contact         ===========");
        println!("=========== (4) Delete contact       ===========");
        println!("=========== (q/Q) Quit               ===========");
        println!("================================================");

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        let mut operation = Operation::Quit;

        if input == "1" {
            operation = Operation::PrintAll;
        } else if input == "2" {
            let mut name = String::new();
            let mut phone_number = String::new();

            println!("Name: ");
            let _ = std::io::stdin().read_line(&mut name);

            println!("Phone Number: ");
            let _ = std::io::stdin().read_line(&mut phone_number);

            let contact = Contact {
                name: name.trim().to_string(),
                phone_number: phone_number.trim().to_string(),
            };

            operation = Operation::AddContact { contact };
        } else if input == "3" {
            let mut old_phone_number = String::new();
            let mut name = String::new();
            let mut phone_number = String::new();

            println!("Old Phone Number: ");
            let _ = std::io::stdin().read_line(&mut old_phone_number);

            println!("New Name: ");
            let _ = std::io::stdin().read_line(&mut name);

            println!("New Phone Number: ");
            let _ = std::io::stdin().read_line(&mut phone_number);

            let contact = Contact {
                name: name.trim().to_string(),
                phone_number: phone_number.trim().to_string(),
            };

            operation = Operation::EditContact {
                phone_number: old_phone_number.trim().to_string(),
                contact: contact,
            };
        } else if input == "4" {
            let mut phone_number = String::new();

            println!("Phone Number: ");
            let _ = std::io::stdin().read_line(&mut phone_number);

            operation = Operation::DeleteContact {
                phone_number: phone_number.trim().to_string(),
            };
        } else if input.to_lowercase() == "q" {
            save_contacts(&contacts);
            operation = Operation::Quit;
        } else {
            println!("Invalid input!");
        }

        do_operation(operation, &mut contacts);

        println!("(q/Q) for to quit or Press Enter to continue...");

        let mut choice = String::new();
        let _ = std::io::stdin().read_line(&mut choice);

        if choice.trim().to_lowercase() == "q" {
            save_contacts(&contacts);
            quit = true;
        }
    }
}
