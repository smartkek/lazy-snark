use crate::error_type::AppResult;
use crate::request_response::Response;

use linked_hash_map::LinkedHashMap;
use serde_json::Value;

use bellman::groth16::{
    prepare_verifying_key, verify_proof, PreparedVerifyingKey, Proof, VerifyingKey,
};
use bellman::pairing::bn256::*;
use bellman::pairing::ff::{Field, PrimeField, PrimeFieldRepr};
use bellman::pairing::{CurveAffine, EncodedPoint};

use byteorder::{BigEndian, WriteBytesExt};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

pub struct ProofManager {
    // map from proof id to verify status
    proofs: LinkedHashMap<u64, u8>,

    // prepared verifying key for proof
    prepared_vk: PreparedVerifyingKey<Bn256>,
}

impl ProofManager {
    pub fn new() -> Self {
        let vk_alpha_g1 = hex::decode(
            "2e0a814dd75e4118233ddf6a916a813c40bae07d976fdcd01dbfa22bea641a96
            1779e77cff5e54cf2cdc237e51cd6d95ef2c37ab6a7d5f9ce0a242188e1a1fe3",
        )
        .unwrap();

        //some g1 point, because ZoKrates does not return that and it is not needed for verification
        let vk_beta_g1 = hex::decode(
            "1fea09defec64586a976a33dbfb70961fc7e03fb6f4d5a1e074f97312ce789cd
            006653d8d2e65ab55fa795c44971eabcc6dbb1dd383c7a8a20de68486eb28154",
        )
        .unwrap();

        let vk_beta_g2 = hex::decode(
            "021548b93199574bdef2be8cb1908a1079b1664d8a041d2e297c3aa6c554855c
            190b2d5d03854400e2c2a702f502813677a1d4be920d79648f810e320a30f2c5
            0bc956fa715451d64e20b260759c2ae74a82b68f1eef86504051cd3ae547f282
            011192ee83c0347e363b7c5fffe156fbadd91591b35dc8fe912d2b498c3a9301",
        )
        .unwrap();

        let vk_gamma_g2 = hex::decode(
            "1c4c46720835faf06e35cd85f05c589a1a98f58112ecf7aacf0deac60681f5a4
            1b438f01daf6402ff298981b74f80a5e79c39cce21c67770f74b89e65eb3b9ca
            101b8c9c29aa1ac1a709878f6eb4d4a74f4ed1368a18f29c2762b76b8c389f4d
            009538b3640e10082d0bf4b18b997fef6af2e7cceb942ebb26bd263e8805fedd",
        )
        .unwrap();

        //some g1 point, because ZoKrates does not return that and it is not needed for verification
        let vk_delta_g1 = hex::decode(
            "1fea09defec64586a976a33dbfb70961fc7e03fb6f4d5a1e074f97312ce789cd
            006653d8d2e65ab55fa795c44971eabcc6dbb1dd383c7a8a20de68486eb28154",
        )
        .unwrap();

        let vk_delta_g2 = hex::decode(
            "25161a4cc549ffabd2c4508038c12d49447c15e9c565b025183ff6114ffcc58b
            110f2b773f6d9632162bc2c629467a58e7539ed0f0dc64ff4fd8f63baf4b5a32
            0eb80be9e5a3f3f4cb0e39edc1db88dbf8de59b0c800b72dcc34d9c0fae14d55
            0839d69bfc27640a59af741138d4f34500d925eb1a4e9fd57fcda269a7411c33",
        )
        .unwrap();
        
        let vk_ic_len = 6;

        let vk_ic = hex::decode(
            "2bb604557c5f1096973ab8afe980ea3ae23bd7457f3f11f67fb395f2d1f3b568
            0f12fdb646ea572637ea6e1bbf04158bcabe6947cf614c67efb3f0278279f866
            228bbefb9d7457c97766bcae9412c6ddd1de8e3dbcf1606ca6b8f027836affee
            01bf2712a663f5a72a469ea83a4c3d453c6023a0cd5d5f86330157f1505d62b3
            23af3409b4b3fb3f194dc683be70c5e442de55544edeace8f891a891a4701ca3
            1d13edb38da07247e70158557cfa93097d90d92b9a2c99f190c1413f3fdf8828
            00572fbfedfe16fd1dcae266bf009907451cd8db485325ad322fb658cb0c30ff
            25415b150b181b2cbecc6f84382b0bd8fd49f2cf498da1c775ad624e5e7b7eaf
            1a294f13fbf284a6e11c2f54ed2946fc5fd732dafbf49ac01ce741f224b57c29
            182d4a788849c87d27548cbe3a511a0237cb0d4595425eee878d78c4eb4e5529
            10ec12d1090de44b1aecb41030d123df2d61318c1928d6de10f916c9bfc2f681
            0621a1ea9bbbfa893358dfaa206ba1cb8af2ecca483c3c36f2a0c302da401c8f",
        )
        .unwrap();

        let mut c = Cursor::new(Vec::new());

        c.write_all(vk_alpha_g1.as_slice()).unwrap();
        c.write_all(vk_beta_g1.as_slice()).unwrap();
        c.write_all(vk_beta_g2.as_slice()).unwrap();
        c.write_all(vk_gamma_g2.as_slice()).unwrap();
        c.write_all(vk_delta_g1.as_slice()).unwrap();
        c.write_all(vk_delta_g2.as_slice()).unwrap();
        c.write_i32::<BigEndian>(vk_ic_len).unwrap();
        c.write_all(vk_ic.as_slice()).unwrap();

        c.seek(SeekFrom::Start(0)).unwrap();

        let vk: VerifyingKey<Bn256> = VerifyingKey::read(c).unwrap();
        let prepared_vk: PreparedVerifyingKey<Bn256> = prepare_verifying_key(&vk);

        ProofManager {
            proofs: LinkedHashMap::new(),
            prepared_vk,
        }
    }

    pub fn verify(
        &mut self,
        proof_id: u64,
        public_par: [String; 5],
        proof: [String; 8],
    ) -> AppResult<Value> {
        let mut c = Cursor::new(Vec::new());

        // write a, b, c
        for proof_part in &proof {
            c.write_all(hex::decode(proof_part)?.as_slice())?;
        }
        c.seek(SeekFrom::Start(0)).unwrap();

        let mut g1_repr = <bellman::pairing::bn256::G1Affine as CurveAffine>::Uncompressed::empty();
        let mut g2_repr = <bellman::pairing::bn256::G2Affine as CurveAffine>::Uncompressed::empty();

        c.read_exact(g1_repr.as_mut())?;
        let a = g1_repr
            .into_affine()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            .and_then(|e| {
                if e.is_zero() {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "point at infinity",
                    ))
                } else {
                    Ok(e)
                }
            })?;

        c.read_exact(g2_repr.as_mut())?;
        let b = g2_repr
            .into_affine()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            .and_then(|e| {
                if e.is_zero() {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "point at infinity",
                    ))
                } else {
                    Ok(e)
                }
            })?;

        c.read_exact(g1_repr.as_mut())?;
        let c = g1_repr
            .into_affine()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            .and_then(|e| {
                if e.is_zero() {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "point at infinity",
                    ))
                } else {
                    Ok(e)
                }
            })?;

        let proof: Proof<Bn256> = Proof { a, b, c };

        // import public inputs -----------------------------------------------

        let in_1 = hex::decode(&public_par[0])?;
        let in_2 = hex::decode(&public_par[1])?;
        let in_3 = hex::decode(&public_par[2])?;
        let in_4 = hex::decode(&public_par[3])?;
        let in_5 = hex::decode(&public_par[4])?;

        let mut repr_in_1 = Fr::zero().into_repr();
        repr_in_1.read_be(&in_1[..])?;
        let in_1_fr = Fr::from_repr(repr_in_1)?;

        let mut repr_in_2 = Fr::zero().into_repr();
        repr_in_2.read_be(&in_2[..])?;
        let in_2_fr = Fr::from_repr(repr_in_2)?;

        let mut repr_in_3 = Fr::zero().into_repr();
        repr_in_3.read_be(&in_3[..])?;
        let in_3_fr = Fr::from_repr(repr_in_3)?;

        let mut repr_in_4 = Fr::zero().into_repr();
        repr_in_4.read_be(&in_4[..])?;
        let in_4_fr = Fr::from_repr(repr_in_4)?;

        let mut repr_in_5 = Fr::zero().into_repr();
        repr_in_5.read_be(&in_5[..])?;
        let in_5_fr = Fr::from_repr(repr_in_5)?;

        let public_inputs = vec![in_1_fr, in_2_fr, in_3_fr, in_4_fr, in_5_fr];

        let is_valid = verify_proof(&self.prepared_vk, &proof, &public_inputs)?;

        // update proof status ------------------------------------------------

        let result: u8 = is_valid.into();

        // TODO: check that such proof is already in the map (?)
        self.proofs.insert(proof_id, result);

        let response = Response::Verify { result };
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
