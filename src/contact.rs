use crate::{Contacts, PhoneNumber};
use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub name: String,
    pub phone_number: PhoneNumber,
}

impl Contact {
    pub fn print(&self) {
        println!("Name: {}, Phone Num: {}", self.name, self.phone_number)
    }
}

fn load_file(file_url: &String) -> Result<String, Error> {
    let mut file = File::open(file_url.as_str());
    let mut file_content = String::new();

    let _ = match &mut file {
        Ok(f) => {
            let metadata = f.metadata().expect("Error reading contact file");

            if metadata.len() == 0 {
                file_content = String::from("")
            } else {
                let _ = f.read_to_string(&mut file_content)?;
            }
        }
        Err(_) => file_content = String::from("Error"),
    };
    Ok(file_content)
}

pub fn load_contacts() -> Result<Contacts, Error> {
    let contacts_file_source = String::from("contacts.json");
    let contacts_string = load_file(&contacts_file_source).expect("Error loading file");
    let mut contacts = Contacts::new();
    if contacts_string != "" && contacts_string != "Error" {
        contacts = serde_json::from_str(&*contacts_string)
            .expect("Error deserializing contacts.json file");
    }
    Ok(contacts)
}

pub fn save_contacts(contacts: &Contacts) {
    let contacts_file_source = "contacts.json";

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(contacts_file_source)
        .expect("Error opening contacts.json file");

    let contacts_json =
        serde_json::to_string(&contacts).expect("Error serializing contacts.json file");

    file.set_len(0).expect("Error clearing file");
    file.write_all(contacts_json.as_bytes()).expect("Error writing contacts.json file");
}

pub fn print_contacts(contacts: &Contacts) {
    println!("================================================");
    if contacts.is_empty() {
        println!("There is no contact!")
    } else {
        println!("All the {} contacts!", contacts.len());

        println!("================================================");

        for (_, contact) in contacts.iter() {
            contact.print();
        }
    }
    println!("================================================");
}

pub fn add_contact(contact: Contact, contacts: &mut Contacts) {
    contacts.insert(contact.phone_number.clone(), contact);
}

pub fn edit_contact(phone_number: PhoneNumber, contact: Contact, contacts: &mut Contacts) {
    if contacts.get(phone_number.as_str()).is_none() {
        contacts.insert(phone_number.clone(), contact);
    }
}

pub fn delete_contact(phone_number: PhoneNumber, contacts: &mut Contacts) {
    if contacts.get(phone_number.as_str()).is_none() {
        contacts.remove(phone_number.as_str());
    }
}
