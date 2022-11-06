# Introduction

This is a demonstration on using [Arkworks](https://github.com/arkworks-rs) library to customize the constraint systems, create proof and verify the proof.
The constraint system that I'm refering is from Polygon's zkEVM [Memory State Circuit](https://docs.hermez.io/zkEVM/zkProver/State-Machines/Secondary-State-Machines/Memory/Memory/#complete-example).

## A basic walkthrough of what the project does

1. Create a MemoryCircuit struct which refer to Polygon's zkEVM Memory State Circuit.
2. Implement Constraint System's generation function that has the flexibility to create any constraint. (I only create one constraint for demonstration).
3. Generate Proof based on given table's data (in JSON format).
4. Verify the Proof.

# Tech stack

There are many cryptography/circuit packages or libraries available to choose like [circom](https://docs.circom.io/), [ZK-Garage](https://github.com/ZK-Garage/plonk), [jellyfish/plonk](https://github.com/EspressoSystems/jellyfish/tree/main/plonk/src),etc. However, I'm chosing Arkworks because of their modular structure(easy to pick the library needed) and it is fully written in Rust.

Proving System: [Groth16](https://eprint.iacr.org/2016/260.pdf)
Elliptic Curve: [BLS](https://en.wikipedia.org/wiki/BLS_digital_signature).

# Dev

1. `cargo build` : Build the folder

2. `cargo run` : Run the code under `main.rs`

3. `cargo test` : For testing

For more information, check out [Rust book](https://doc.rust-lang.org/book/).

# Future Plan

[x] Testing with different Elliptic Curve.
[] Write proof using other arkworks package.
[] Build front end for user to interact with, customize input and parameters from front end
[] Create real world application
