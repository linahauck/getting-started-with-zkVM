#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

//This guest code to check if a person is 18 or older 

fn main() {

    // read the input
    let age: u32 = env::read();

    // Logic circuit
    assert!(age >= 18, "The person is underage!");

    // write public output to the journal
    //TODO: make this a msg such as "This person is at lest 18 y.o." (env::commit(&("msg"))) instead of using println
    env::commit(&("This is a valid proof that this person is at least 18 y.o."));
}
