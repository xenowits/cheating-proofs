use std::fs::OpenOptions;
use std::io::{Error, Read, Write};
use hello_xenowits_methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, serde::{to_vec, from_slice}};

fn main() {
    let suspected_contacts = vec![String::from("1")];
    let call_history = vec![String::from("1"), String::from("2")];
    let receipt_filename = String::from("receipt");

    match create_receipt(receipt_filename.clone(), suspected_contacts, call_history) {
        Ok(_) => println!("Receipt created successfully!"),
        Err(e) => panic!("Error creating receipt: {}", e),
    }

    match verify_receipt(receipt_filename) {
        Ok(cheating) => {
            if cheating {
                println!("Your boyfriend IS cheating.")
            } else {
                println!("Your boyfriend is NOT cheating.")
            }
        },
        Err(e) => panic!("verify receipt: {}", e),
    };
}

// create_receipt creates a zk-receipt of cheating proof and saves the receipt to the provided file.
pub fn create_receipt(receipt_filename: String, suspected_contacts: Vec<String>, call_history: Vec<String>) -> Result<Receipt, Error> {
    // First, we construct an executor environment
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .add_input(&to_vec(&suspected_contacts).unwrap())
        .add_input(&to_vec(&call_history).unwrap())
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, MULTIPLY_ELF).unwrap();

    // Serialize receipt using bincode.
    let serialized_receipt = bincode::serialize(&receipt).unwrap();

    // Save serialized receipt to file.
    let mut receipt_file = OpenOptions::new().write(true).create(true).open(receipt_filename).unwrap();
    receipt_file.write_all(&*serialized_receipt).unwrap();

    println!("Len of serialized receipt: {}", serialized_receipt.len());

    return Ok(receipt);
}

// verify_receipt loads a receipt from the provided file and verifies the receipt.
pub fn verify_receipt(receipt_file: String) -> Result<bool, Error> {
    // Read the bytes from file.
    let mut receipt_file = OpenOptions::new().read(true).open(receipt_file)?;
    let mut read_bytes = vec![];
    receipt_file.read_to_end(&mut read_bytes)?;

    // Deserialize receipt.
    let decoded: Receipt = bincode::deserialize(&read_bytes[..]).unwrap();

    // Verify receipt.
    decoded.verify(MULTIPLY_ID).unwrap();

    let cheating: bool = from_slice(&decoded.journal).unwrap();

    return Ok(cheating);
}