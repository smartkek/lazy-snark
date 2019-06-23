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

        //////////////// vk

        let vk_byte = "
        vk.alpha = 0x2c2cb1812fb05d4f31791c66ff995d756b73162f3bb016a5c114befe5cd7903e, 0x0abc1f8a5d49cb2dbda15b5a8b7cd81bec0a581e7c2e16f79446af2d2f5340c0
        vk.beta = [0x071644533641f7e3acb8606328c591853b2bc27253f29bc11d008a67996fc07f, 0x26ca2720c073a085d8452aef541aac280879971c09b199a6e0f21bf36745e1d8], [0x0b17104896ed701b6d52279992c1f20d558bc0de8284087645633bf3ca1a0c98, 0x2c10eb5b6c0ca42ede8cdcf60642c6dca040abe9abb8294948f4aa0be59a0d42]
        vk.gamma = [0x0afbadec2ecafdd62278c7021095660f5786f445c040e628e4ed1a410454b582, 0x038aa6f04ee254a97e2b75ea1f30e36785b6cde4dfd3a2371e058ce089b9ad51], [0x077720bb216fb0051c5e153c1bd9aa36a678173b9c13e8d3a83cb5a75ca36948, 0x1f9b58e9abde296abc3c3bab8fb0be2a4f497d8e5d9d463997d316e9cc558a7d]
        vk.delta = [0x16526b9b519fa544d3f9ce35a5f4afa7aac0aa4dd54421c4864b3fe8d2415f41, 0x24e24f35699cca59416a7f43c0e93e148b2353440978994df8f81603a46f8839], [0x299f9f09280310aedf63055c5ce76feb16557ed7ff11ba35adad718102b5651a, 0x0c4a2fc4db77ef6c19511b2ffb369981cebbffcb5337a671e1ad678b460ac5e9]
        vk.gammaABC.len() = 6
        vk.gammaABC[0] = 0x2f910078bf5092a7ea9d3ce750b7b5399b101509adb8017a6e12fa1a4c638d5b, 0x0b76454d4300571c8d86714b4e5ef095688b51080e674425e8e5edb201f64128
        vk.gammaABC[1] = 0x2922a307d415f70c8df6f14b664c46df12a89cd3a89cf7960663907bf9483b68, 0x1bc30a719ddc0099f557cca61a0687766e6275fc98b185baa77735b93bf2a0ef
        vk.gammaABC[2] = 0x0201dc8c8faa3dc5b8eec85f029d2482bf11a6b46d5f8e4d9f17d41ac3e4c9c1, 0x0a1d62c1142c92dff75b53d5a572fd7a013708118acf10f718c61fb6226160f5
        vk.gammaABC[3] = 0x02276f5896610ec573cd6cdc6e47c69e756362d2b1b1c51c5ab90ac838d1a898, 0x13fa6cc7987f4f3118f6ee3ab85dcd708df17050636d487914077348e0af05b1
        vk.gammaABC[4] = 0x0778ae3718fd7f48564bc33b60ec4f39a238e97cb4cc0bbd4ff37119942ff7d4, 0x0282e96481744ee21524d802b3e524bf0596bb37bb63e5ed37c77fc1a5c8e89d
        vk.gammaABC[5] = 0x1ce40e230695bdab7d2ff7ebcf6e6fedb68d1a320238fc98845b151ae4ee3b54, 0x0feac76664d37b57a4ea5a774252bb82355294e55635a8aeb7a1327405d27128";

        let mut c = Cursor::new(Vec::new());

        c.write_all(vk_byte.as_bytes()).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();

        let vk = VerifyingKey::<E>::read(&mut c)?;
        
        let pvk = prepare_verifying_key(&vk);

        //////////////// proof

        let proof_byte = "
        a: [0x12d0dbcfc1da3ea29bc017288fceea3929401f4f12dbd0bba73781420d31aa2d, 0x2811c1eaa63f4a804951bd7f994cbb6bea9df64591793b8392400e8756d1bca7],
        b: [[0x04c33f68e1bd55be0928b086c647debcdf7aa0e3c3efc6a8efbc2596a77a0e67, 0x17e7392e0e3ec2b5701e675e6e0569330d03ffffe476fc8d63cfeaa0ba1c8a97], [0x2fc402693a54cd1b176abeed209674f2f12ced1496c6ce27ba8cf16903daa4cc, 0x2c47efba3f4f260da643bb6427d08b551bb3446537d6ac4857d611be2355a446]],
        c: [0x04d40f14694092d0f70890a20492b2b68e7eaabdcee744e519678d687c9c3ed0, 0x28de140e393154b0e70b3ef12806af963a4a33b45c24e7864391093b6028fa2b]";

        let mut c2 = Cursor::new(Vec::new());

        c2.write_all(proof_byte.as_bytes()).unwrap();
        c2.seek(SeekFrom::Start(0)).unwrap();

        let proof =  Proof::<E>::read(&mut c2);

        /////////////// pub_input

        let pub_input = "inputs: [0x00000000000000000000000000000000c6481e22c5ff4164af680b8cfaa5e8ed, \
        0x000000000000000000000000000000003120eeff89c4f307c4a6faaae059ce10, \
        0x000000000000000000000000000000005b6d7d198c48c17c9540d29275a04662, \
        0x00000000000000000000000000000000f7a9aa434629a33c84eec3e16e196f27, \
        0x0000000000000000000000000000000000000000000000000000000000000001]";

        result_bool = verify_proof(
            &pvk,
            &proof,
            pub_input
        ).unwrap();

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
