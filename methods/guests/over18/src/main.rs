#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

//This guest code to check if a person is 18 or older 

fn main() {

    // read the input
    let age: u32 = env::read();

    // Logic circuit
    let is_valid = &(age >= 18);
    assert!(is_valid, "UNVALID PROOF: This person is not be over 18!");

    env::commit(&("VALID PROOF: This person is at least 18 years old."));
}
