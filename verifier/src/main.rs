use std::fs::OpenOptions;
use std::io::{Error, Read};
use hello_xenowits_methods::{MULTIPLY_ID};
use risc0_zkvm::{Receipt, serde::{from_slice}};

fn main() {
    let receipt_filename = String::from("receipt");

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