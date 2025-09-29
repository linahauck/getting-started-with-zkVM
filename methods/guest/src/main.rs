use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // Initialize the logger
    //env_logger::init();

    // Log messages
    //info!("This is an info message.");

    // TODO: do something with the input

    // write public output to the journal
    env::commit(&input);
}
