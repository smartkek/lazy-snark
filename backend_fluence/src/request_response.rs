use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum Request {
    Verify {
        proof_id: u64,
        public_par: [String; 5],
        proof: [String; 8],
    },
    Check {
        proof_id: u64,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Verify { result: u8 },
    Check { verified: u8 },
    Error { message: String },
}
