use crate::error_type::AppResult;
use crate::request_response::Response;

use linked_hash_map::LinkedHashMap;
use serde_json::Value;

pub struct ProofManager {
    // map from job id to verify status
    proofs: LinkedHashMap<u64, u8>,
}

impl ProofManager {

    pub fn new() -> Self {
        ProofManager {
            proofs: LinkedHashMap::new(),
        }
    }

    pub fn verify(&mut self, proof_id: u64, public_par: [f64; 5], proof: [f64; 8]) -> AppResult<Value> {

        use bellman::pairing::bn256::*;
        use bellman::groth16::{
            Proof,
            VerifyingKey,
            verify_proof, 
            prepare_verifying_key
        };

        use bellman::pairing::ff::{
            PrimeField,
            PrimeFieldRepr,
            Field
        };

        use bellman::pairing::{
            CurveAffine,
            EncodedPoint
        };

        use std::io::{self, Read, Cursor, Seek, SeekFrom, Write};

        // verify mock
        let mut result: u8 = 1;
        if proof[0] != 8.51065254754666e75 {
            result = 0
        }

        // verify proof -------------------------------------------------------------------------------------

        // import verification key --------------------------------------------

        //vk.alpha_g1
        let mut hex_string = hex::decode("2c2cb1812fb05d4f31791c66ff995d756b73162f3bb016a5c114befe5cd7903e").unwrap();
        hex_string.append(hex::decode("0abc1f8a5d49cb2dbda15b5a8b7cd81bec0a581e7c2e16f79446af2d2f5340c0").unwrap().as_mut());
        //vk.beta_g1 = vk.alpha_g1 - ZoKrates does not return that and it is not neaded for verification
        hex_string.append(hex::decode("1fea09defec64586a976a33dbfb70961fc7e03fb6f4d5a1e074f97312ce789cd").unwrap().as_mut());
        hex_string.append(hex::decode("006653d8d2e65ab55fa795c44971eabcc6dbb1dd383c7a8a20de68486eb28154").unwrap().as_mut());
        //vk.beta_g2
        hex_string.append(hex::decode("071644533641f7e3acb8606328c591853b2bc27253f29bc11d008a67996fc07f").unwrap().as_mut());
        hex_string.append(hex::decode("26ca2720c073a085d8452aef541aac280879971c09b199a6e0f21bf36745e1d8").unwrap().as_mut());
        hex_string.append(hex::decode("0b17104896ed701b6d52279992c1f20d558bc0de8284087645633bf3ca1a0c98").unwrap().as_mut());
        hex_string.append(hex::decode("2c10eb5b6c0ca42ede8cdcf60642c6dca040abe9abb8294948f4aa0be59a0d42").unwrap().as_mut());
        //vk.gamma_g2
        hex_string.append(hex::decode("0afbadec2ecafdd62278c7021095660f5786f445c040e628e4ed1a410454b582").unwrap().as_mut());
        hex_string.append(hex::decode("038aa6f04ee254a97e2b75ea1f30e36785b6cde4dfd3a2371e058ce089b9ad51").unwrap().as_mut());
        hex_string.append(hex::decode("077720bb216fb0051c5e153c1bd9aa36a678173b9c13e8d3a83cb5a75ca36948").unwrap().as_mut());
        hex_string.append(hex::decode("1f9b58e9abde296abc3c3bab8fb0be2a4f497d8e5d9d463997d316e9cc558a7d").unwrap().as_mut());
        //vk.delta_g1 = vk.alpha_g1 - ZoKrates does not return that and it is not neaded for verification
        hex_string.append(hex::decode("1fea09defec64586a976a33dbfb70961fc7e03fb6f4d5a1e074f97312ce789cd").unwrap().as_mut());
        hex_string.append(hex::decode("006653d8d2e65ab55fa795c44971eabcc6dbb1dd383c7a8a20de68486eb28154").unwrap().as_mut());
        //vk.delta_g2
        hex_string.append(hex::decode("16526b9b519fa544d3f9ce35a5f4afa7aac0aa4dd54421c4864b3fe8d2415f41").unwrap().as_mut());
        hex_string.append(hex::decode("24e24f35699cca59416a7f43c0e93e148b2353440978994df8f81603a46f8839").unwrap().as_mut());
        hex_string.append(hex::decode("299f9f09280310aedf63055c5ce76feb16557ed7ff11ba35adad718102b5651a").unwrap().as_mut());
        hex_string.append(hex::decode("0c4a2fc4db77ef6c19511b2ffb369981cebbffcb5337a671e1ad678b460ac5e9").unwrap().as_mut());
        //vk.ic len
        hex_string.append(6u32.to_be_bytes().to_vec().as_mut());
        //vk.ic
        hex_string.append(hex::decode("2f910078bf5092a7ea9d3ce750b7b5399b101509adb8017a6e12fa1a4c638d5b").unwrap().as_mut());
        hex_string.append(hex::decode("0b76454d4300571c8d86714b4e5ef095688b51080e674425e8e5edb201f64128").unwrap().as_mut());
        hex_string.append(hex::decode("2922a307d415f70c8df6f14b664c46df12a89cd3a89cf7960663907bf9483b68").unwrap().as_mut());
        hex_string.append(hex::decode("1bc30a719ddc0099f557cca61a0687766e6275fc98b185baa77735b93bf2a0ef").unwrap().as_mut());
        hex_string.append(hex::decode("0201dc8c8faa3dc5b8eec85f029d2482bf11a6b46d5f8e4d9f17d41ac3e4c9c1").unwrap().as_mut());
        hex_string.append(hex::decode("0a1d62c1142c92dff75b53d5a572fd7a013708118acf10f718c61fb6226160f5").unwrap().as_mut());

        hex_string.append(hex::decode("02276f5896610ec573cd6cdc6e47c69e756362d2b1b1c51c5ab90ac838d1a898").unwrap().as_mut());
        hex_string.append(hex::decode("13fa6cc7987f4f3118f6ee3ab85dcd708df17050636d487914077348e0af05b1").unwrap().as_mut());
        hex_string.append(hex::decode("0778ae3718fd7f48564bc33b60ec4f39a238e97cb4cc0bbd4ff37119942ff7d4").unwrap().as_mut());
        hex_string.append(hex::decode("0282e96481744ee21524d802b3e524bf0596bb37bb63e5ed37c77fc1a5c8e89d").unwrap().as_mut());
        hex_string.append(hex::decode("1ce40e230695bdab7d2ff7ebcf6e6fedb68d1a320238fc98845b151ae4ee3b54").unwrap().as_mut());
        hex_string.append(hex::decode("0feac76664d37b57a4ea5a774252bb82355294e55635a8aeb7a1327405d27128").unwrap().as_mut());

        let mut c = Cursor::new(Vec::new());

        c.write_all(hex_string.as_slice()).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();

        let vk : VerifyingKey<Bn256> = VerifyingKey::read(c).unwrap();

        let prepared_vk = prepare_verifying_key(&vk);

        // import proof -------------------------------------------------------

        // a
        let mut hex_string = hex::decode("12d0dbcfc1da3ea29bc017288fceea3929401f4f12dbd0bba73781420d31aa2d").unwrap();
        hex_string.append(hex::decode("2811c1eaa63f4a804951bd7f994cbb6bea9df64591793b8392400e8756d1bca7").unwrap().as_mut());
        // b
        hex_string.append(hex::decode("04c33f68e1bd55be0928b086c647debcdf7aa0e3c3efc6a8efbc2596a77a0e67").unwrap().as_mut());
        hex_string.append(hex::decode("17e7392e0e3ec2b5701e675e6e0569330d03ffffe476fc8d63cfeaa0ba1c8a97").unwrap().as_mut());
        hex_string.append(hex::decode("2fc402693a54cd1b176abeed209674f2f12ced1496c6ce27ba8cf16903daa4cc").unwrap().as_mut());
        hex_string.append(hex::decode("2c47efba3f4f260da643bb6427d08b551bb3446537d6ac4857d611be2355a446").unwrap().as_mut());
        
        // c
        hex_string.append(hex::decode("04d40f14694092d0f70890a20492b2b68e7eaabdcee744e519678d687c9c3ed0").unwrap().as_mut());
        hex_string.append(hex::decode("28de140e393154b0e70b3ef12806af963a4a33b45c24e7864391093b6028fa2b").unwrap().as_mut());

        let mut c = Cursor::new(Vec::new());

        c.write_all(hex_string.as_slice()).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();

        let mut g1_repr = <bellman::pairing::bn256::G1Affine as CurveAffine>::Uncompressed::empty();
        let mut g2_repr = <bellman::pairing::bn256::G2Affine as CurveAffine>::Uncompressed::empty();

        c.read_exact(g1_repr.as_mut()).unwrap();
        let a = g1_repr
                    .into_affine()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                    .and_then(|e| if e.is_zero() {
                        Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                    } else {
                        Ok(e)
                    }).unwrap();

        c.read_exact(g2_repr.as_mut()).unwrap();
        let b = g2_repr
                    .into_affine()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                    .and_then(|e| if e.is_zero() {
                        Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                    } else {
                        Ok(e)
                    }).unwrap();

        c.read_exact(g1_repr.as_mut()).unwrap();
        let c = g1_repr
                    .into_affine()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                    .and_then(|e| if e.is_zero() {
                        Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                    } else {
                        Ok(e)
                    }).unwrap();

        let proof : Proof<Bn256> = Proof {a: a, b: b, c: c};

        // import public inputs -----------------------------------------------

        let in_1 = hex::decode("00000000000000000000000000000000c6481e22c5ff4164af680b8cfaa5e8ed").unwrap();
        let in_2 = hex::decode("000000000000000000000000000000003120eeff89c4f307c4a6faaae059ce10").unwrap();
        let in_3 = hex::decode("000000000000000000000000000000005b6d7d198c48c17c9540d29275a04662").unwrap();
        let in_4 = hex::decode("00000000000000000000000000000000f7a9aa434629a33c84eec3e16e196f27").unwrap();
        let in_5 = hex::decode("0000000000000000000000000000000000000000000000000000000000000001").unwrap();

        let mut repr_in_1 = Fr::zero().into_repr();
        repr_in_1.read_be(&in_1[..]).expect("pack new hash as field element");
        let in_1_fr = Fr::from_repr(repr_in_1).expect("must be a valud new representation");

        let mut repr_in_2 = Fr::zero().into_repr();
        repr_in_2.read_be(&in_2[..]).expect("pack new hash as field element");
        let in_2_fr = Fr::from_repr(repr_in_2).expect("must be a valud new representation");

        let mut repr_in_3 = Fr::zero().into_repr();
        repr_in_3.read_be(&in_3[..]).expect("pack new hash as field element");
        let in_3_fr = Fr::from_repr(repr_in_3).expect("must be a valud new representation");

        let mut repr_in_4 = Fr::zero().into_repr();
        repr_in_4.read_be(&in_4[..]).expect("pack new hash as field element");
        let in_4_fr = Fr::from_repr(repr_in_4).expect("must be a valud new representation");

        let mut repr_in_5 = Fr::zero().into_repr();
        repr_in_5.read_be(&in_5[..]).expect("pack new hash as field element");
        let in_5_fr = Fr::from_repr(repr_in_5).expect("must be a valud new representation");

        let public_inputs = vec![in_1_fr, in_2_fr, in_3_fr, in_4_fr, in_5_fr];

        let is_valid = verify_proof(&prepared_vk, &proof, &public_inputs).expect("must verify a proof");

        // update proof status
        self.proofs.insert(proof_id, result);

        let response = Response::Verify {
            result : result,
        };

        serde_json::to_value(response).map_err(Into::into)
    }

    pub fn check(&self, proof_id: u64) -> AppResult<Value> {
        let status = self.proof_status(proof_id)?;
        let response = Response::Check { verified: status };

        serde_json::to_value(response).map_err(Into::into)
    }

    fn proof_status(&self, proof_id: u64) -> AppResult<u8> {
        let status = self
            .proofs
            .get(&proof_id)
            .ok_or_else(|| format!("Proof with id {} wasn't found", proof_id))?;

        Ok(*status)
    }
}
