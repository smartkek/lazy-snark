extern crate bellman;
extern crate sapling_crypto;
extern crate crypto;
extern crate rand;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

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

#[derive(Clone)]
pub struct ModifyStateWitness {
    pub old_first_value_bits: Vec<Option<bool>>,
    pub old_second_value_bits: Vec<Option<bool>>,
    pub new_first_value_bits: Vec<Option<bool>>,
    pub new_second_value_bits: Vec<Option<bool>>,
}

pub struct ConfidentialState<'a, E: JubjubEngine> {
    pub params: &'a E::Params,
    pub old_state: Option<E::Fr>,
    pub new_state: Option<E::Fr>,
    pub witness: ModifyStateWitness,
}

impl<'a, E:JubjubEngine + 'a> Clone for ConfidentialState<'a, E> {
    fn clone(&self) -> Self {
        ConfidentialState {
            params: self.params,
            old_state: self.old_state.clone(),
            new_state: self.new_state.clone(),
            witness: self.witness.clone(),
        }
    }
}

const NUM_VALUE_BITS: usize = 128;

impl<'a, E: JubjubEngine> Circuit<E> for ConfidentialState<'a, E> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError>
    {
        assert_eq!(self.witness.old_first_value_bits.len(), NUM_VALUE_BITS);
        assert_eq!(self.witness.old_second_value_bits.len(), NUM_VALUE_BITS);
        assert_eq!(self.witness.new_first_value_bits.len(), NUM_VALUE_BITS);
        assert_eq!(self.witness.new_second_value_bits.len(), NUM_VALUE_BITS);

        // Expose the old truncated hash as the input
        let old_hash = num::AllocatedNum::alloc(
            cs.namespace(|| "old account state"),
            || {
                let value = self.old_state;
                Ok(*value.get()?)
            }
        )?;
        old_hash.inputize(cs.namespace(|| "old state input"))?;

        // Expose the new truncated hash as the input
        let new_hash = num::AllocatedNum::alloc(
            cs.namespace(|| "new account state"),
            || {
                let value = self.new_state;
                Ok(*value.get()?)
            }
        )?;
        new_hash.inputize(cs.namespace(|| "new state input"))?;


        // -------------------------------------------------------------------------------------- value
        let mut old_first_value_bits = Vec::<boolean::Boolean>::with_capacity(NUM_VALUE_BITS);

        for i in 0..NUM_VALUE_BITS {
            let bit = boolean::Boolean::from(boolean::AllocatedBit::alloc(
                cs.namespace(|| format!("allocating old first value bit {}", i)),
                self.witness.old_first_value_bits[i]
                )?
            );
            old_first_value_bits.push(bit);
        }

        /*let mut coeff = E::Fr::one();
        let mut old_first_num_lc = num::Num::<E>::zero();
        for bit in old_first_value_bits.iter() {
            old_first_num_lc = old_first_num_lc.add_bool_with_coeff(CS::one(), bit, coeff);
            coeff.double();
        }

        let old_first_value = num::AllocatedNum::alloc(
            cs.namespace(|| "old first value"), 
            || {
                let value = old_first_num_lc.get_value();
                Ok(*value.get()?)
            }
        )?;*/

        let mut old_second_value_bits = Vec::<boolean::Boolean>::with_capacity(NUM_VALUE_BITS);

        for i in 0..NUM_VALUE_BITS {
            let bit = boolean::Boolean::from(boolean::AllocatedBit::alloc(
                cs.namespace(|| format!("allocating old second value bit {}", i)),
                self.witness.old_second_value_bits[i]
                )?
            );
            old_second_value_bits.push(bit);
        }

        /*let mut coeff = E::Fr::one();
        let mut old_second_num_lc = num::Num::<E>::zero();
        for bit in old_second_value_bits.iter() {
            old_second_num_lc = old_second_num_lc.add_bool_with_coeff(CS::one(), bit, coeff);
            coeff.double();
        }

        let old_second_value = num::AllocatedNum::alloc(
            cs.namespace(|| "old second value"), 
            || {
                let value = old_second_num_lc.get_value();
                Ok(*value.get()?)
            }
        )?;*/

        let mut new_first_value_bits = Vec::<boolean::Boolean>::with_capacity(NUM_VALUE_BITS);

        for i in 0..NUM_VALUE_BITS {
            let bit = boolean::Boolean::from(boolean::AllocatedBit::alloc(
                cs.namespace(|| format!("allocating new first value bit {}", i)),
                self.witness.new_first_value_bits[i]
                )?
            );
            new_first_value_bits.push(bit);
        }

        /*let mut coeff = E::Fr::one();
        let mut new_first_num_lc = num::Num::<E>::zero();
        for bit in new_first_value_bits.iter() {
            new_first_num_lc = new_first_num_lc.add_bool_with_coeff(CS::one(), bit, coeff);
            coeff.double();
        }

        let new_first_value = num::AllocatedNum::alloc(
            cs.namespace(|| "new first value"), 
            || {
                let value = new_first_num_lc.get_value();
                Ok(*value.get()?)
            }
        )?;*/

        let mut new_second_value_bits = Vec::<boolean::Boolean>::with_capacity(NUM_VALUE_BITS);

        for i in 0..NUM_VALUE_BITS {
            let bit = boolean::Boolean::from(boolean::AllocatedBit::alloc(
                cs.namespace(|| format!("allocating new second value bit {}", i)),
                self.witness.new_second_value_bits[i]
                )?
            );
            new_second_value_bits.push(bit);
        }

        /*let mut coeff = E::Fr::one();
        let mut new_second_num_lc = num::Num::<E>::zero();
        for bit in new_second_value_bits.iter() {
            new_second_num_lc = new_second_num_lc.add_bool_with_coeff(CS::one(), bit, coeff);
            coeff.double();
        }

        let new_second_value = num::AllocatedNum::alloc(
            cs.namespace(|| "new second value"), 
            || {
                let value = new_second_num_lc.get_value();
                Ok(*value.get()?)
            }
        )?;*/

        // enforce sum is equal!

        /*cs.enforce(
            || "enforce sum is equal",
            |_| old_first_value.get_variable() + old_second_value.get_variable(),
            || CS::one(),
            |_| new_first_value.get_variable() + new_second_value.get_variable()
        );*/

        // calculate the hash value

        old_first_value_bits.extend(old_second_value_bits);

        let mut calculated_old_hash = sha256::sha256(
            cs.namespace(|| "calculate old state hash"),
            &old_first_value_bits
        )?;

        calculated_old_hash.reverse();
        calculated_old_hash.truncate(E::Fr::CAPACITY as usize);

        let mut old_packed_hash_lc = num::Num::<E>::zero();
        let mut coeff = E::Fr::one();
        for bit in calculated_old_hash {
            old_packed_hash_lc = old_packed_hash_lc.add_bool_with_coeff(CS::one(), &bit, coeff);
            coeff.double();
        }

        cs.enforce(
            || "enforce old hash equality to external input",
            |lc| lc + old_hash.get_variable(),
            |lc| lc + CS::one(),
            |_| old_packed_hash_lc.lc(E::Fr::one())
        );

        new_first_value_bits.extend(new_second_value_bits);

        let mut calculated_new_hash = sha256::sha256(
            cs.namespace(|| "calculate new state hash"),
            &new_first_value_bits
        )?;

        calculated_new_hash.reverse();
        calculated_new_hash.truncate(E::Fr::CAPACITY as usize);

        let mut new_packed_hash_lc = num::Num::<E>::zero();
        let mut coeff = E::Fr::one();
        for bit in calculated_new_hash {
            new_packed_hash_lc = new_packed_hash_lc.add_bool_with_coeff(CS::one(), &bit, coeff);
            coeff.double();
        }

        cs.enforce(
            || "enforce new hash equality to external input",
            |lc| lc + new_hash.get_variable(),
            |lc| lc + CS::one(),
            |_| new_packed_hash_lc.lc(E::Fr::one())
        );

        Ok(())
    }
}

fn be_bit_vector_into_bytes
    (bits: &Vec<bool>) -> Vec<u8>
{
    let mut bytes: Vec<u8> = vec![];
    for byte_chunk in bits.chunks(8)
    {
        let mut byte = 0u8;
        // pack just in order
        for (i, bit) in byte_chunk.iter().enumerate()
        {
            if *bit {
                byte |= 1 << (7 - i);
            }
        }
        bytes.push(byte);
    }

    bytes
}

fn u128_into_be_bits(value: u128) -> Vec<bool>
{
    let mut v = value;
    let mut bits: Vec<bool> = vec![];
    for _ in 0..128 {
        if v & 1 > 0 {
            bits.push(true);
        } else {
            bits.push(false);
        }
        v >>= 1;
    }
    bits.reverse();

    bits
}

pub fn le_bit_vector_into_field_element<P: PrimeField>
    (bits: &Vec<bool>) -> P
{
    // double and add
    let mut fe = P::zero();
    let mut base = P::one();

    for bit in bits {
        if *bit {
            fe.add_assign(&base);
        }
        base.double();
    }

    fe
}

pub fn encode_fs_into_fr<E: JubjubEngine>(input: E::Fs)
    -> E::Fr 
{
    let mut fs_le_bits: Vec<bool> = BitIterator::new(input.into_repr()).collect();
    fs_le_bits.reverse();

    let converted = le_bit_vector_into_field_element::<E::Fr>(&fs_le_bits);

    converted
}

pub fn print_to_file(p: &str, buf: &[u8]) {
    let path = Path::new(p);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all(buf) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               why.description())
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn main() {
    use bellman::pairing::bn256::*;
    use rand::{SeedableRng, Rng, XorShiftRng, Rand};
    use sapling_crypto::circuit::test::*;
    use sapling_crypto::alt_babyjubjub::{AltJubjubBn256, fs, edwards, PrimeOrder};
    use bellman::groth16::{generate_random_parameters, create_random_proof, verify_proof, prepare_verifying_key};
    use crypto::sha2::Sha256;
    use crypto::digest::Digest;
    use std::path::PathBuf;

    let params = &AltJubjubBn256::new();

    let mut rng = XorShiftRng::from_seed([0x3dbe6258, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    let (public_inputs, circuit) = {
        let old_first_value: u64 = rng.gen(); // Rand is not implemented for u128, so we can away with it
        let old_first_value = old_first_value as u128;
        let old_first_value_bits = u128_into_be_bits(old_first_value);

        let old_second_value: u64 = rng.gen();
        let old_second_value = old_second_value as u128;
        let old_second_value_bits = u128_into_be_bits(old_second_value);

        let new_first_value: u64 = rng.gen();
        let new_first_value = new_first_value as u128;
        let new_first_value_bits = u128_into_be_bits(new_first_value);

        let new_second_value: u64 = rng.gen();
        let new_second_value = new_second_value as u128;
        let new_second_value_bits = u128_into_be_bits(new_second_value);

        let witness = ModifyStateWitness {
            old_first_value_bits: old_first_value_bits.iter().map(|el| Some(*el)).collect(),
            old_second_value_bits: old_second_value_bits.iter().map(|el| Some(*el)).collect(),
            new_first_value_bits: new_first_value_bits.iter().map(|el| Some(*el)).collect(),
            new_second_value_bits: new_second_value_bits.iter().map(|el| Some(*el)).collect(),
        };

        let mut old_hash_bits = old_first_value_bits.clone();
        old_hash_bits.extend(old_second_value_bits.clone());
        let old_bytes_to_hash: Vec<u8> = be_bit_vector_into_bytes(&old_hash_bits);

        let mut new_hash_bits = new_first_value_bits.clone();
        new_hash_bits.extend(new_second_value_bits.clone());
        let new_bytes_to_hash: Vec<u8> = be_bit_vector_into_bytes(&new_hash_bits);

        let mut old_hash_result = [0u8; 32];
        // calculate a hash and repack it as field element for witness
        let mut old_h = Sha256::new();
        old_h.input(&old_bytes_to_hash);
        old_h.result(&mut old_hash_result[..]);

        let mut new_hash_result = [0u8; 32];

        let mut new_h = Sha256::new();
        new_h.input(&new_bytes_to_hash);
        new_h.result(&mut new_hash_result[..]);

        let bits_to_trim = 256 - Fr::CAPACITY;
        let trimming_mask = (1u8 << (8 - bits_to_trim)) - 1u8;

        assert_eq!(trimming_mask, 0x1f);

        // truncate the top bits if this hash to later use it as BE representation of a field element
        old_hash_result[0] &= trimming_mask; // trim top 3 bits for BN256 case.
        new_hash_result[0] &= trimming_mask;

        let mut old_repr = Fr::zero().into_repr();
        old_repr.read_be(&old_hash_result[..]).expect("pack old hash as field element");
        let old_state = Fr::from_repr(old_repr).expect("must be a valud old representation");

        let mut new_repr = Fr::zero().into_repr();
        new_repr.read_be(&new_hash_result[..]).expect("pack new hash as field element");
        let new_state = Fr::from_repr(new_repr).expect("must be a valud new representation");

        let circuit = ConfidentialState::<Bn256> {
            params: &params,
            old_state: Some(old_state),
            new_state: Some(new_state),
            witness: witness
        };

        (vec![old_state, new_state], circuit)
    };

    // TestConstraintSystem is a special constraint system implementation that does not reduce R1CS
    // and eagerly calculates all closures
    {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let circuit = circuit.clone();

        circuit.synthesize(&mut cs).expect("circuit must synthesize");
        // we can use internal tools to check for some variables being unconstrained (e.g. declared, but not used)
        let unconstrained = cs.find_unconstrained();
        println!("{}", unconstrained);
        assert!(unconstrained == "");

        // let's check that our constraints are satisfied with a current assignment
        let unsatisfied = cs.which_is_unsatisfied();
        if unsatisfied.is_some() {
            panic!("{}", unsatisfied.unwrap());
        }
        println!("Constraint system is satisfied");
    };

    // we can generate parameters on an empty circuit actually!
    // closures compute the witness, but constraints do not depend on them
    let parameters = {
            let witness = ModifyStateWitness {
                old_first_value_bits: vec![None; NUM_VALUE_BITS],
                old_second_value_bits: vec![None; NUM_VALUE_BITS],
                new_first_value_bits: vec![None; NUM_VALUE_BITS],
                new_second_value_bits: vec![None; NUM_VALUE_BITS],
            };

            let circuit = ConfidentialState::<Bn256> {
                params: &params,
                old_state: None,
                new_state: None,
                witness: witness
            };

        generate_random_parameters(circuit, &mut rng).expect("must generate parameters")
    };

    // write vk in binary form
    let vk_file = File::create(PathBuf::from("./vk.key")).unwrap();
    parameters.vk.write(vk_file).unwrap();

    let prepared_vk = prepare_verifying_key(&parameters.vk);

    // now let's make a proof
    let proof = {
        create_random_proof(circuit, &parameters, &mut rng).expect("must create a proof")
    };

    // write proof in binary form
    let proof_file = File::create(PathBuf::from("./proof.key")).unwrap();
    proof.write(proof_file).unwrap();

    println!("public inputs: {}", public_inputs[0]);

    let is_valid = verify_proof(&prepared_vk, &proof, &public_inputs).expect("must verify a proof");

    assert!(is_valid, "proof was invalid");
    println!("Test is complete");
}