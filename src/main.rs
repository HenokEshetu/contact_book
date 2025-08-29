type PhoneNumber = String;
type Contacts = Vec<Contact>;


struct Contact {
    name: String,
    phone_number: PhoneNumber
}


impl Contact {
    fn print(&self) {
        println!("Name: {}, Phone Num: {}", self.name, self.phone_number)
    }
}

enum Operation {
    AddContact { contact: Contact },
    EditContact { phone_number: PhoneNumber, contact: Contact },
    DeleteContact { phone_number: PhoneNumber },
    PrintAll,
    Quit
}

fn print_contacts(contacts: &Contacts) {
    if contacts.is_empty() {
        println!("There is no contact!")
    } else {
        println!("All the {} contacts!", contacts.len());

        for contact in contacts.iter() {
            contact.print();
        }
    }
}


fn add_contact(contact: Contact, contacts: &mut Contacts) {
    contacts.push(contact)
}

fn edit_contact(phone_number: PhoneNumber, contact: Contact, contacts: &mut Contacts) {
    if let Some(i) = contacts.iter().position(|_contact| _contact.phone_number == phone_number) {
        contacts[i] = contact
    }
}

fn delete_contact(phone_number: PhoneNumber, contacts: &mut Contacts) {
    if let Some(i) = contacts.iter().position(|_contact| _contact.phone_number == phone_number) {
        contacts.remove(i);
    }
}

fn do_operation(op: Operation, contacts: &mut Contacts) {
    match op {
        Operation::AddContact { contact } => add_contact(contact, contacts),
        Operation::EditContact { phone_number, contact } => edit_contact(phone_number, contact, contacts),
        Operation::DeleteContact { phone_number } => delete_contact(phone_number, contacts),
        Operation::PrintAll => print_contacts(&contacts),
        Operation::Quit => std::process::exit(0),
    }
}

fn main() {
    let mut contacts = Contacts::new();
    let mut quit = false;

    while !quit {
        println!("================================================");
        println!("=========== Welcome to Contact Book ============");
        println!("================================================");
        println!("(1) Print all contact");
        println!("(2) Add contact");
        println!("(3) Edit contact");
        println!("(4) Delete contact");
        println!("(q/Q) Quit");
        println!("> ");

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
                phone_number: phone_number.trim().to_string()
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
                phone_number: phone_number.trim().to_string()
            };

            operation = Operation::EditContact { 
                phone_number: old_phone_number.trim().to_string(), 
                contact: contact
            };
        } else if input == "4" {
            let mut phone_number = String::new();

            println!("Phone Number: ");
            let _ = std::io::stdin().read_line(&mut phone_number);

            operation = Operation::DeleteContact { 
                phone_number: phone_number.trim().to_string()
            };
        } else if input.to_lowercase() == "q" {
            operation = Operation::Quit;
        } else {
            println!("Invalid input!");
        }

        do_operation(operation, &mut contacts);

        println!("(q/Q) for to quit or Press Enter to continue...");

        let mut choice = String::new();
        let _ = std::io::stdin().read_line(&mut choice);

        if choice.trim().to_lowercase() == "q" {
            quit = true;
        }
    }
}
