use std::ops::{MulAssign, SubAssign};

use ark_ec::PairingEngine;
use ark_ff::{Field, Zero};
use ark_relations::{
    lc,
    r1cs::{
        ConstraintSynthesizer, ConstraintSystemRef, Result as R1CSResult, SynthesisError, Variable,
    },
};
//use ark_ff::UniformRand;
use ark_groth16::*;
use ark_std::{test_rng, UniformRand};
pub struct MemoryCircuit<F: Field> {
    // ISNOLAST: Option<F>,
    // address: Option<F>,
    // step: Option<F>,
    mOp: Option<F>,
    mWr: Option<F>,
    // lastAccess: usize,
    // val0: usize,
    // val1: usize,
    // val2: usize,
    // val3: usize,
    // val4: usize,
    // val5: usize,
    // val6: usize,
    // val7: usize,
}

impl<ConstraintF: Field> ConstraintSynthesizer<ConstraintF> for MemoryCircuit<ConstraintF> {
    // Generate constraint below
    // Constraint: (1-mOp)*(mWr)=0
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // let mOp = Field::from(self.mOp);
        // let mWr = ConstraintF::from(self.mWr);
        let mOp = cs.new_witness_variable(|| self.mOp.ok_or(SynthesisError::AssignmentMissing))?;
        let mWr = cs.new_witness_variable(|| self.mWr.ok_or(SynthesisError::AssignmentMissing))?;
        // let one = ConstraintF::from(1u64);
        // let out = cs.new_input_variable(|| Ok(Fr::from(0u64))).unwrap();

        let out = cs.new_input_variable(|| {
            let mut one = ConstraintF::one();
            let mOp = self.mOp.ok_or(SynthesisError::AssignmentMissing)?;
            let mWr = self.mWr.ok_or(SynthesisError::AssignmentMissing)?;
            one.sub_assign(&mOp);
            one.mul_assign(&mWr);
            Ok(one)
        })?;
        cs.enforce_constraint(
            lc!() + (ConstraintF::one(), Variable::One) - mOp,
            lc!() + mWr,
            lc!() + out,
        )?;
        // cs.finalize();
        Ok(())
    }
}

pub fn prove_and_verify<E: PairingEngine>(n_iters: usize) {
    let rng = &mut test_rng();
    let parameters: ProvingKey<E> = generate_random_parameters::<E, _, _>(
        MemoryCircuit {
            mOp: None,
            mWr: None,
        },
        rng,
    )
    .unwrap();

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
                mOp: Some(mOp),
                mWr: Some(mWr),
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
