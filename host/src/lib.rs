use anyhow::{Context, Result};
use risc0_zkvm::{ExecutorEnv, Receipt, default_prover};
use methods::{OVER18_ELF}; //TODO: JWT_VALIDATOR_ELF


// By running this someone can produce a receipt that proves that 
// they are over 18 years old

// pub fn confirm_jwt_integrity(token: &str) -> Result<Receipt> {
//     let env = ExecutorEnv::builder()
//         .write(&token)
//         .unwrap()
//         .build()
//         .unwrap();

//     let prover = default_prover();

//     let receipt = prover.prove(env, JWT_VALIDATOR_ELF).with_context(|| format!("Guest program failed. There is no valid proof of confirmed integrity."))?.receipt;
//     Ok(receipt)
// }

pub fn not_underage(age: u32) -> Result<Receipt> {
    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // A default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    let env = ExecutorEnv::builder()
        .write(&age)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let receipt = prover.prove(env, OVER18_ELF).with_context(|| format!("Guest program failed. There is no valid proof of being over 18."))?.receipt;
    Ok(receipt)
}

use const_random::const_random  ;
//cmd to run tests: cargo test 
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn is_18() {
        const TEST_FACTOR : u32 = 18;
        let result = 
        not_underage(TEST_FACTOR);
        
        assert!(result.is_ok(), "Expected OK for 18yo person!");
    }

    #[test]
    fn is_over_18() {
        const TEST_FACTOR : u32 = random_age_generator(19,100);
        let result = 
        not_underage(TEST_FACTOR);

        assert!(result.is_ok(), "Expected OK for person over 18!");
    }

    #[test]
    fn is_underage() {
        const TEST_FACTOR : u32 = random_age_generator(0,17);
        let result = 
        not_underage(TEST_FACTOR);
        
        assert!(result.is_err(),  "Expected a panic for underage person!");
    }
}

pub const fn random_age_generator(min: u32, max: u32) -> u32 {

    const RANDOM_NUMBER: u32 = const_random!(u32);
    
    let age_range_size = max - min + 1;
    (RANDOM_NUMBER % age_range_size) + min
}
