use crate::Contact;
use crate::{add_contact, delete_contact, edit_contact, print_contacts, Contacts, PhoneNumber};

pub enum Operation {
    AddContact { contact: Contact },
    EditContact { phone_number: PhoneNumber, contact: Contact },
    DeleteContact { phone_number: PhoneNumber },
    PrintAll,
    Quit
}

pub fn do_operation(op: Operation, contacts: &mut Contacts) {
    match op {
        Operation::AddContact { contact } => add_contact(contact, contacts),
        Operation::EditContact { phone_number, contact } => edit_contact(phone_number, contact, contacts),
        Operation::DeleteContact { phone_number } => delete_contact(phone_number, contacts),
        Operation::PrintAll => print_contacts(&contacts),
        Operation::Quit => std::process::exit(0),
    }
}
