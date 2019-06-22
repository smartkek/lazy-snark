use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Verify {
        proof_id: u64,
        public_par: [u64; 5],
        proof: [u64; 8],
    },
    Check {
        proof_id: u64,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Verify { result: u8 },
    Check { verifed: u8 },
    Error { message: String },
}
