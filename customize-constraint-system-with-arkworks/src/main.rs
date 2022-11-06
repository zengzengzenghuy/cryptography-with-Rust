mod cs_gen;
use crate::cs_gen::{prove_and_verify, random_prove_and_verify, MemoryCircuit};

use ark_bls12_377::Bls12_377;
use ark_cp6_782::CP6_782;

fn main() {
    // Test groth16 prove and verification with different Elliptic Curve
    // Randomly generate variable for testing
    // BLS Elliptic Curve
    random_prove_and_verify::<Bls12_377>(1);
    // CP Elliptic Curve
    random_prove_and_verify::<CP6_782>(1);

    // Test groth16 prove and verification with different Elliptic Curve
    // Read data from Json file
    // BLS Elliptic Curve
    prove_and_verify::<Bls12_377>();
    // CP Elliptic Curve
    prove_and_verify::<CP6_782>();
}
