use std::fs::OpenOptions;
use std::io::{Read, Write};
use hello_xenowits_methods::{MULTIPLY_ELF, MULTIPLY_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt, serde::{to_vec, from_slice}};

fn main() {
    let a: u64 = 17;
    let b: u64 = 23;

   // First, we construct an executor environment
   let env = ExecutorEnv::builder()
     // Send a & b to the guest
     .add_input(&to_vec(&a).unwrap())
     .add_input(&to_vec(&b).unwrap())
     .build()
     .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove_elf(env, MULTIPLY_ELF).unwrap();

    // TODO: Implement code for transmitting or serializing the receipt for
    // other parties to verify here
    let serialized_receipt = bincode::serialize(&receipt).unwrap();

    // Save serialized output to file
    let receipt_filename = "receipt.txt";
    let mut receipt_file = OpenOptions::new().write(true).create(true).open(receipt_filename).unwrap();
    println!("Len of serialized receipt: {}", serialized_receipt.len());
    receipt_file.write_all(&*serialized_receipt).unwrap();

    // Optional: Verify receipt to confirm that recipients will also be able to
    // verify your receipt
    receipt.verify(MULTIPLY_ID).unwrap();

    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = from_slice(&receipt.journal).unwrap();

    // Print an assertion
    println!("Hello, world! I know the factors of {}, and I can prove it!", c);

    // Let's deserialize the bytes from file
    let mut receipt_file = OpenOptions::new().read(true).open(receipt_filename).unwrap();
    let mut read_bytes = vec![];
    let x = receipt_file.read_to_end(&mut read_bytes).unwrap();
    println!("X: {}, {}", x, receipt_filename);
    let decoded: Receipt = bincode::deserialize(&read_bytes[..]).unwrap();

    assert_eq!(receipt.journal, decoded.journal);

    println!("Cool guys!!");
}
