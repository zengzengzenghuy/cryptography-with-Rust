mod cs_gen;
use crate::cs_gen::{prove_and_verify, random_prove_and_verify, MemoryCircuit};

use ark_bls12_377::Bls12_377;
use ark_cp6_782::CP6_782;

fn main() {
    println!("Hello, world!");

    random_prove_and_verify::<Bls12_377>(1);
    random_prove_and_verify::<CP6_782>(1);

    prove_and_verify::<Bls12_377>();
    prove_and_verify::<CP6_782>();
}
