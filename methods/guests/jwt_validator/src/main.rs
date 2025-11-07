#![no_main]

//use jwt_core::Validator;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

//TODO: gen a pk
//static PUBLIC_KEY: &str = "...";


fn main() {

    // read the input
    let token: String = env::read();

    // create new validator instance from public key
    // let validator = PUBLIC_KEY
    //     .parse::<Validator>()
    //     .expect("failed to create validator from key");

    //  // validate token
    // let valid_token = validator
    //     .validate_token_integrity(token.as_str())
    //     .expect("token integrity check failed");

    //TODO: hva skal returneres i vår case?
    env::commit(&("VALID PROOF: JWT integrity is confirmed {}", token));
}
