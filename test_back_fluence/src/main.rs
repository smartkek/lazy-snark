extern crate bellman;
extern crate sapling_crypto;
extern crate crypto;
extern crate rand;

use bellman::pairing::ff::{
    PrimeField,
    PrimeFieldRepr,
    Field,
    BitIterator
};

use bellman::pairing::{
    Engine
};

use bellman::{
    SynthesisError,
    ConstraintSystem,
    Circuit
};

use sapling_crypto::circuit::{
    Assignment,
    boolean,
    ecc,
    sha256,
    num,
    multipack,
};

use sapling_crypto::jubjub::{
    JubjubEngine,
    FixedGenerators,
    PrimeOrder,
    Unknown,
    edwards,
    JubjubParams
};

fn main() {
    use bellman::pairing::bn256::*;
    use rand::{SeedableRng, Rng, XorShiftRng, Rand};
    use sapling_crypto::circuit::test::*;
    use sapling_crypto::alt_babyjubjub::{AltJubjubBn256, fs, edwards, PrimeOrder};
    use bellman::groth16::{verify_proof, prepare_verifying_key};
    use crypto::sha2::Sha256;
    use crypto::digest::Digest;

    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::path::PathBuf;
    use bellman::groth16::Proof;
    use bellman::groth16::VerifyingKey;


    // import verification key



    self.alpha_g1.into_uncompressed().as_ref();

    //let vk_file = File::open(PathBuf::from("./verification.key")).unwrap();
    //let vk : VerifyingKey<Bn256> = VerifyingKey::read(vk_file).unwrap();

    //let vk_file = File::open(PathBuf::from("./vk.key")).unwrap();
    //let vk : VerifyingKey<Bn256> = VerifyingKey::read(vk_file).unwrap();

    //let prepared_vk = prepare_verifying_key(&vk);

    // import proof
    //let proof_file = File::open(PathBuf::from("./proof.key")).unwrap();
    //let proof : Proof<Bn256> = Proof::read(proof_file).unwrap(); 

    // import public inputs
    //let public_inputs = vec![];

    //let is_valid = verify_proof(&prepared_vk, &proof, &public_inputs).expect("must verify a proof");

    //assert!(is_valid, "proof was invalid");
    println!("Proof is valid");
}