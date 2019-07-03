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

    use bellman::pairing::{
    Engine,
    CurveAffine,
    EncodedPoint
    };


    use crate::{
        SynthesisError
    };

    use std::io::{self, Read};
    use std::sync::Arc;

    // import verification key

    //let vk_file = File::open(PathBuf::from("./verification.key")).unwrap();
    //let vk : VerifyingKey<Bn256> = VerifyingKey::read(vk_file).unwrap();

    //let vk_file = File::open(PathBuf::from("./vk.key")).unwrap();
    //let vk : VerifyingKey<Bn256> = VerifyingKey::read(vk_file).unwrap();

    //let prepared_vk = prepare_verifying_key(&vk);

    // import proof
    let proof_file = File::open(PathBuf::from("./proof.bin")).unwrap();
    let proof : Proof<Bn256> = Proof::read(proof_file).unwrap(); 

    // export proof
    let mut proof_file = File::create(PathBuf::from("./proof2.bin")).unwrap();
    proof_file.write_all(proof.a.into_uncompressed().as_ref());
    //proof_file.write_all(proof.b.into_compressed().as_ref());
    //0x125c6452b97d9b83600e903313aa93284a8b8d8318050ed6af65c10df7ebb5ec
    //let [u8] = 
    //proof_file.write_all(proof.c.into_compressed().as_ref());

    let mut proof_file = File::open(PathBuf::from("./proof2.bin")).unwrap();
    //let proof : Proof<Bn256> = Proof::read(proof_file).unwrap();


    let mut g1_repr = <bellman::pairing::bn256::G1Affine as CurveAffine>::Uncompressed::empty();
    //let mut g2_repr = <bellman::pairing::bn256::G2Affine as CurveAffine>::Uncompressed::empty();

    proof_file.read_exact(g1_repr.as_mut());
    //let a = g1_repr.into_affine();
    let a = g1_repr
                .into_affine()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                .and_then(|e| if e.is_zero() {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                } else {
                    Ok(e)
                }).unwrap();
    println!("!!! proof a g1: {}", a);


     // write proof.a in hex form and binary form
    let a : G1Affine = proof.a;
    println!("proof a g1: {}", a);
    println!("proof a g1 compressed: {:?}", a.into_compressed().as_ref());

    let b : G2Affine = proof.b;
    println!("proof b g2: {}", b);
    println!("proof b g2 compressed: {:?}", b.into_compressed().as_ref());

    let c : G1Affine = proof.c;
    println!("proof c g1: {}", c);
    println!("proof c g1 uncompressed: {:?}", c.into_compressed().as_ref());

    // import public inputs
    //let public_inputs = vec![];

    //let is_valid = verify_proof(&prepared_vk, &proof, &public_inputs).expect("must verify a proof");

    //assert!(is_valid, "proof was invalid");
    println!("Proof is valid");
}