use std::{fs::File, ops::MulAssign};

use ark_ec::PairingEngine;
use ark_ff::Field;
use ark_relations::{
    lc,
    r1cs::{
        ConstraintSynthesizer, ConstraintSystemRef, Result as R1CSResult, SynthesisError, Variable,
    },
};
//use ark_ff::UniformRand;
use ark_groth16::*;
use ark_std::{test_rng, UniformRand};
use serde_json::{from_reader, Value};

// Struct for memory state machine
// each element represents each column from table
// Check for reference: https://docs.hermez.io/zkEVM/zkProver/State-Machines/Secondary-State-Machines/Memory/Memory/#complete-example
pub struct MemoryCircuit<F: Field> {
    ISNOLAST: Option<F>,
    address: Option<F>,
    step: Option<F>,
    mOp: Option<F>,
    mWr: Option<F>,
    lastAccess: Option<F>,
    val0: Option<F>,
    val1: Option<F>,
    val2: Option<F>,
    val3: Option<F>,
    val4: Option<F>,
    val5: Option<F>,
    val6: Option<F>,
    val7: Option<F>,
}

// implement ConstraintSynthesizer trait from arkworks' ark-relations/r1cs
impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for MemoryCircuit<ConstraintF> {
    // Customizing the constraint system by calling generate_constraints function
    // in here the constraint that we want to generate is
    // Constraint: (1-mOp)*(mWr)=0
    // mOp: bool = memory operation
    // mWr: bool = memory write
    // This constraint is checking whether mOp is 1 when mWr is 1
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // witness variable is also the private variable
        let mOp = cs.new_witness_variable(|| self.mOp.ok_or(SynthesisError::AssignmentMissing))?;
        let mWr = cs.new_witness_variable(|| self.mWr.ok_or(SynthesisError::AssignmentMissing))?;

        // input variable is also the publicvariable
        let out = cs.new_input_variable(|| {
            let mut one = ConstraintF::one();
            let mOp = self.mOp.ok_or(SynthesisError::AssignmentMissing)?;
            let mWr = self.mWr.ok_or(SynthesisError::AssignmentMissing)?;
            one.sub_assign(&mOp);
            one.mul_assign(&mWr);
            Ok(one)
        })?;
        // record the constraint into the Constraint System
        cs.enforce_constraint(
            lc!() + (ConstraintF::one(), Variable::One) - mOp,
            lc!() + mWr,
            lc!() + out,
        )?;

        Ok(())
    }
}

// Generate prove and verify using arkworks' groth16 package
// the element of Memory Circuit is generated randomly
// n_iters: number of iteration for prove and verify
pub fn random_prove_and_verify<E: PairingEngine>(n_iters: usize) {
    let rng = &mut test_rng();
    // generate proving key
    let parameters: ProvingKey<E> = generate_random_parameters::<E, _, _>(
        MemoryCircuit {
            ISNOLAST: None,
            address: None,
            step: None,
            mOp: None,
            mWr: None,
            lastAccess: None,
            val0: None,
            val1: None,
            val2: None,
            val3: None,
            val4: None,
            val5: None,
            val6: None,
            val7: None,
        },
        rng,
    )
    .unwrap();
    // generate verifying key
    let pvk = prepare_verifying_key::<E>(&parameters.vk);

    for i in 0..n_iters {
        println!("Iteration {:#?}", i);
        let mOp = E::Fr::rand(rng);
        let mWr = E::Fr::rand(rng);

        let one = E::Fr::from(1u64);

        let mut out = one - mOp;
        out.mul_assign(&mWr);

        let proof = create_random_proof(
            MemoryCircuit {
                ISNOLAST: None,
                address: None,
                step: None,
                mOp: Some(mOp),
                mWr: Some(mWr),
                lastAccess: None,
                val0: None,
                val1: None,
                val2: None,
                val3: None,
                val4: None,
                val5: None,
                val6: None,
                val7: None,
            },
            &parameters,
            rng,
        )
        .unwrap();
        let verify_proof_result: R1CSResult<bool> = verify_proof(&pvk, &proof, &[out]);
        // should be true if verified correctly
        println!("{:#?}", verify_proof_result.unwrap());
        // verify_proof return false if public input is wrong because 'mOp' and 'mWr' is private input
        assert!(!verify_proof(&pvk, &proof, &[mOp]).unwrap());
    }
}

// Generate prove and verify using arkworks' groth16 package
// the element of Memory Circuit is generated from json file under "./src/data/memory-table.json"
pub fn prove_and_verify<E: PairingEngine>() {
    // Read data
    let data_file = File::open("./src/data/memory-table.json").unwrap();
    let json: serde_json::Value =
        serde_json::from_reader(data_file).expect("file should be proper JSON");
    let data = json.get("data").expect("file should have data key");

    let rng = &mut test_rng();
    let parameters: ProvingKey<E> = generate_random_parameters::<E, _, _>(
        MemoryCircuit {
            ISNOLAST: None,
            address: None,
            step: None,
            mOp: None,
            mWr: None,
            lastAccess: None,
            val0: None,
            val1: None,
            val2: None,
            val3: None,
            val4: None,
            val5: None,
            val6: None,
            val7: None,
        },
        rng,
    )
    .unwrap();

    let pvk = prepare_verifying_key::<E>(&parameters.vk);
    let array_len = data.as_array().unwrap().len();
    for i in 0..array_len {
        println!("Iteration {:#?}", i);
        let mOp = E::Fr::from(data.as_array().unwrap()[i]["mOp"].as_u64().unwrap());
        let mWr = E::Fr::from(data.as_array().unwrap()[i]["mWr"].as_u64().unwrap());

        let one = E::Fr::from(1u64);

        let mut out = one - mOp;
        out.mul_assign(&mWr);

        let proof = create_random_proof(
            MemoryCircuit {
                ISNOLAST: None,
                address: None,
                step: None,
                mOp: Some(mOp),
                mWr: Some(mWr),
                lastAccess: None,
                val0: None,
                val1: None,
                val2: None,
                val3: None,
                val4: None,
                val5: None,
                val6: None,
                val7: None,
            },
            &parameters,
            rng,
        )
        .unwrap();
        let verify_proof_result: R1CSResult<bool> = verify_proof(&pvk, &proof, &[out]);
        println!("{:#?}", verify_proof_result.unwrap());
        assert!(!verify_proof(&pvk, &proof, &[mOp]).unwrap());
    }
}
