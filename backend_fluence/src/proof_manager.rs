use crate::error_type::AppResult;
use crate::request_response::Response;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use bellman_ce::groth16::*;

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

        // verify mock
        let mut result: u8 = 1;
        if proof[0] != 8.51065254754666e75 {
            result = 0
        }

        /*
        let tt = "99999";

        let mut c = Cursor::new(Vec::new());

        // Write into the "file" and seek to the beginning
        c.write_all(tt.as_bytes()).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();



        
        let pvk = {

        } 

        let proof = {

        }

        let pub_input = {

        }

        result_bool = verify_proof(
            &pvk,
            &proof,
            &[Fr::one()]
        ).unwrap();*/

        // update proof status
        self.proofs.insert(proof_id, result);

        let response = Response::Verify {
            result : result,
        };

        serde_json::to_value(response).map_err(Into::into)
    }

    /*pub fn read<R: Read>(
        mut reader: R
    ) -> io::Result<Self>
    {
        let mut g1_repr = <E::G1Affine as CurveAffine>::Uncompressed::empty();
        let mut g2_repr = <E::G2Affine as CurveAffine>::Uncompressed::empty();

        reader.read_exact(g1_repr.as_mut())?;
        let alpha_g1 = g1_repr.into_affine().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        reader.read_exact(g1_repr.as_mut())?;
        let beta_g1 = g1_repr.into_affine().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        reader.read_exact(g2_repr.as_mut())?;
        let beta_g2 = g2_repr.into_affine().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        reader.read_exact(g2_repr.as_mut())?;
        let gamma_g2 = g2_repr.into_affine().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        reader.read_exact(g1_repr.as_mut())?;
        let delta_g1 = g1_repr.into_affine().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        reader.read_exact(g2_repr.as_mut())?;
        let delta_g2 = g2_repr.into_affine().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let ic_len = reader.read_u32::<BigEndian>()? as usize;

        let mut ic = vec![];

        for _ in 0..ic_len {
            reader.read_exact(g1_repr.as_mut())?;
            let g1 = g1_repr
                .into_affine()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                .and_then(|e| if e.is_zero() {
                    Err(io::Error::new(io::ErrorKind::InvalidData, "point at infinity"))
                } else {
                    Ok(e)
                })?;

            ic.push(g1);
        }

        Ok(VerifyingKey {
            alpha_g1: alpha_g1,
            beta_g1: beta_g1,
            beta_g2: beta_g2,
            gamma_g2: gamma_g2,
            delta_g1: delta_g1,
            delta_g2: delta_g2,
            ic: ic
        })
    }*/


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
