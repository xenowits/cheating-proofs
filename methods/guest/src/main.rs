#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
// #![no_std]  // std support is experimental

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use risc0_zkvm::guest::env;
use std::{collections::HashMap, println};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Load the suspected contacts from the host
    let suspected_contacts: Vec<String> = env::read();
    let call_history: Vec<String> = env::read();

    if suspected_contacts.len() == 0 {
        panic!("no suspected contacts")
    } else if call_history.len() == 0 {
        panic!("no call history")
    }

    let is_cheating = cheating(suspected_contacts, call_history);

    env::commit(&is_cheating);
}

/// cheating returns true if call history contains any suspected contact.
fn cheating(suspected_contacts: Vec<String>, call_history: Vec<String>) -> bool {
    let mut hashes = HashMap::new();

    for contact in suspected_contacts {
        hashes.insert(contact, true);
    }

    for contact in call_history {
        match hashes.get(&contact) {
            Some(x) => {
                println!("Found a suspected contact in call history: {}", x);
                
                return true;
            }
            _ => {}
        }
    }

    return true;
}