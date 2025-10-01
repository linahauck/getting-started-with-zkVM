#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


fn main() {
    //The guest code to check if a person is 18 or older 

    // read the input
    let input: u32 = env::read();

    // Logic circuit with the input 
    assert!(input >= 18, "The person is underage!");

    // write public output to the journal
    //TODO: make this a msg such as "This person is at lest 18 y.o." (env::commit(&("msg"))) instead of using println
    env::commit(&input);
}
