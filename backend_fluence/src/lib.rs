mod error_type;
mod proof_manager;
mod request_response;

use crate::error_type::AppResult;
use crate::proof_manager::ProofManager;
use crate::request_response::{Request, Response};

use fluence::sdk::*;
use log::info;
use serde_json::Value;
use std::cell::RefCell;

fn init() {
    logger::WasmLogger::init_with_level(log::Level::Info).unwrap();
}

thread_local! {
    static PROOF_MANAGER: RefCell<ProofManager> = RefCell::new(ProofManager::new());
}

fn do_request(req: String) -> AppResult<Value> {
    let request: Request = serde_json::from_str(req.as_str())?;

    match request {
        Request::Verify {
            proof_id,
            public_par,
            proof,
        } => PROOF_MANAGER.with(|gm| gm.borrow_mut().verify(proof_id, public_par, proof)),

        Request::Check { proof_id } => PROOF_MANAGER.with(|gm| gm.borrow_mut().check(proof_id)),
    }
}

#[invocation_handler(init_fn = init)]
fn main(req: String) -> String {
    info!("req string: {}", req);
    match do_request(req) {
        Ok(res) => res.to_string(),
        Err(err) => {
            let response = Response::Error {
                message: err.to_string(),
            };
            serde_json::to_string(&response).unwrap()
        }
    }
}
