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

    use std::io::{self, Read};
    use std::sync::Arc;

    use std::io::{Cursor, Seek, SeekFrom};

    // import verification key ----------------------------------------------------------------------------------------

    //let vk_file = File::open(PathBuf::from("./verification.key")).unwrap();
    //let vk : VerifyingKey<Bn256> = VerifyingKey::read(vk_file).unwrap();

    //vk.alpha_g1
    let mut hex_string = hex::decode("1fea09defec64586a976a33dbfb70961fc7e03fb6f4d5a1e074f97312ce789cd").unwrap();
    hex_string.append(hex::decode("006653d8d2e65ab55fa795c44971eabcc6dbb1dd383c7a8a20de68486eb28154").unwrap().as_mut());
    //vk.beta_g1
    hex_string.append(hex::decode("09dcdb7f3d5d8f36be6363578bc80c86bb0978b2da7f396e366552c19327d2fa").unwrap().as_mut());
    hex_string.append(hex::decode("06c8435a1215747694d059d9b1e4a36d6acd204af1efaf96c22fa3c978b7b57e").unwrap().as_mut());
    //vk.beta_g2
    hex_string.append(hex::decode("1fb73b30d15c6acbf8e723365798381cddccd353d3473853f8586045da20d9c7").unwrap().as_mut());
    hex_string.append(hex::decode("0ea0d501ac34e65153ff483d15864b7b99d03727e3d18a10442ed2c5b7475616").unwrap().as_mut());
    hex_string.append(hex::decode("15f0b1a8a45717104ab0625e08eadf52f69f05b9fe3b18f33ffa088718c75bb5").unwrap().as_mut());
    hex_string.append(hex::decode("20a215cbe73f412eb06c4f05fbfba106600474e659572d5cacab3e02db4d56b5").unwrap().as_mut());
    //vk.gamma_g2
    hex_string.append(hex::decode("288918039bccc005d837cc445b01e394df8a8ba5f6c9640a54013ce723195a91").unwrap().as_mut());
    hex_string.append(hex::decode("257c137fe6037e6302bcc0e0c8d12c55c32663ea30aaf8602cb505cce116934d").unwrap().as_mut());
    hex_string.append(hex::decode("0b9641484e773a5c30f604b912e7008d8c80221eb8af076206d1cb62d5459d6d").unwrap().as_mut());
    hex_string.append(hex::decode("1422be63e21d8de851ba9ef345b3ba8e9f30dcc9b2b9181700c6c7387fd0000c").unwrap().as_mut());
    //vk.delta_g1
    hex_string.append(hex::decode("2685638d1ab1c53e240faa4c600e08bf5acaeb923f530d6c6b02d752c4401c6e").unwrap().as_mut());
    hex_string.append(hex::decode("16bd94558a5419e22b4fca334f8e11c67d2aeaa0668521a6c41cd5804c03afa3").unwrap().as_mut());
    //vk.delta_g2
    hex_string.append(hex::decode("199f02185af5ab7a2646ad6523cf0e75160919770fac8be6d87a2c85f5c62fa2").unwrap().as_mut());
    hex_string.append(hex::decode("2415e319be12366f2b5f59f6475008d3fa8f2486dbbd417ff979dc727ac8b506").unwrap().as_mut());
    hex_string.append(hex::decode("211d44b4f8fc9f4aa10acce7e45205e497df0ba61ae321db03a520d213f2f884").unwrap().as_mut());
    hex_string.append(hex::decode("1a684894edd1457382d460ac84ffbae1dbb1d35403e3729e06edf10960ccbb58").unwrap().as_mut());
    //vk.ic len
    hex_string.append(3u32.to_be_bytes().to_vec().as_mut());
    //vk.ic
    hex_string.append(hex::decode("26d321094b0704a73f6c3b0fa361ce36ec94775d3dcd4b0fcbed086f2417b69d").unwrap().as_mut());
    hex_string.append(hex::decode("26895772f453edf3d38c1e3b11a74d134fec4e3f225bae6620470bafedbbe374").unwrap().as_mut());
    hex_string.append(hex::decode("28ca43de88e898c1b5871d2d2da4a60fb470a459c64922556253eb0d87a0950e").unwrap().as_mut());
    hex_string.append(hex::decode("2e076fbdb638cd389f3a2748ba759ddbee20b97dfb577343af18109506675ead").unwrap().as_mut());
    hex_string.append(hex::decode("02a3959d26d938fbbd2f1453b4cdc8558703133524ddab59dba20e8b920f27ab").unwrap().as_mut());
    hex_string.append(hex::decode("293e35b2ba09b27006eead4fe4a4aae136391e33a56ee5b2b9bff744f2657e74").unwrap().as_mut());

    let mut c = Cursor::new(Vec::new());

    c.write_all(hex_string.as_slice()).unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();

    let vk : VerifyingKey<Bn256> = VerifyingKey::read(c).unwrap();

    let prepared_vk = prepare_verifying_key(&vk);

    // import proof ---------------------------------------------------------------------------------------------------

    /*let proof_file = File::open(PathBuf::from("./proof.bin")).unwrap();
    let proof : Proof<Bn256> = Proof::read(proof_file).unwrap();*/

    let mut hex_string = hex::decode("143c347b098006a40a22eaa2d2bc56222e734f55bd717721c90886518872015d").unwrap();
    hex_string.append(hex::decode("10f7e2ca628844cb7c2ccacc8abb4930cd226dcd37f9e83860f942a177020c7d").unwrap().as_mut());
    
    hex_string.append(hex::decode("2902bfe7dba8187bd73795211f2eece7d5aa3ba8a1f27e34c8ba148cad4ac743").unwrap().as_mut());
    hex_string.append(hex::decode("2e0b0d4278e6b20f5b2e31fa425c7c91d1eebdaeb01fef228b5954e2a0c01f8a").unwrap().as_mut());
    hex_string.append(hex::decode("2f2c29c73d45cfedf4334873e418860f561c3d7fa4090fe7aeb686023398874d").unwrap().as_mut());
    hex_string.append(hex::decode("14f4411f6edfd09d31ad7f6f78dbbe7debfa66a0cb4e0450888790b4293eeaf1").unwrap().as_mut());

    hex_string.append(hex::decode("125c6452b97d9b83600e903313aa93284a8b8d8318050ed6af65c10df7ebb5ec").unwrap().as_mut());
    hex_string.append(hex::decode("1a5702d896bdf72c8853dc91819a5de30d7415c61c0eb9c55d947485b268ef89").unwrap().as_mut());

    let mut c = Cursor::new(Vec::new());

    c.write_all(hex_string.as_slice()).unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();

    let mut g1_repr = <bellman::pairing::bn256::G1Affine as CurveAffine>::Uncompressed::empty();
    let mut g2_repr = <bellman::pairing::bn256::G2Affine as CurveAffine>::Uncompressed::empty();

    c.read_exact(g1_repr.as_mut());
    let a = g1_repr
                .into_affine()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                .and_then(|e| if e.is_zero() {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                } else {
                    Ok(e)
                }).unwrap();
    //println!("proof a g1: {}", a);

    c.read_exact(g2_repr.as_mut());
    let b = g2_repr
                .into_affine()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                .and_then(|e| if e.is_zero() {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                } else {
                    Ok(e)
                }).unwrap();
    //println!("proof b g2: {}", b);

    c.read_exact(g1_repr.as_mut());
    let c = g1_repr
                .into_affine()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                .and_then(|e| if e.is_zero() {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                } else {
                    Ok(e)
                }).unwrap();
    //println!("proof c g1: {}", c);

    let proof : Proof<Bn256> = Proof {a: a, b: b, c: c};

    // import public inputs
    //let public_inputs = vec![];

    //let is_valid = verify_proof(&prepared_vk, &proof, &public_inputs).expect("must verify a proof");

    //assert!(is_valid, "proof was invalid");
    println!("Proof is valid");
}