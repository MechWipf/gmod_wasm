mod global_state;
mod wasm;

use lrpc::*;
use lua_wasmer_common::rpc::{
    EchoReq, EchoRes, InvokeReq, InvokeRes, IsValidReq, IsValidRes, NewFromBinaryReq,
    NewFromWatReq, NewRes, RemoveReq, RemoveRes,
};

#[fmt_function]
pub fn handle_echo(req: EchoReq) -> EchoRes {
    EchoRes {
        content: Ok(req.msg),
    }
}

#[fmt_function]
pub fn handle_new_from_wat(req: NewFromWatReq) -> NewRes {
    let instance = match wasm::wasm_instance_str(&req.wat) {
        Ok(x) => x,
        Err(e) => {
            return NewRes {
                content: Err(e.to_string()),
            }
        }
    };

    match global_state::GLOBAL_STATE.lock() {
        Ok(x) => NewRes {
            content: Ok(x.instance_add(instance)),
        },
        Err(e) => NewRes {
            content: Err(String::from("Failed to aquire lock")),
        },
    }
}

#[fmt_function]
pub fn handle_new_from_binary(req: NewFromBinaryReq) -> NewRes {
    let instance = match wasm::wasm_instance_bytes(&req.binary) {
        Ok(x) => x,
        Err(e) => {
            return NewRes {
                content: Err(e.to_string()),
            }
        }
    };

    match global_state::GLOBAL_STATE.lock() {
        Ok(x) => NewRes {
            content: Ok(x.instance_add(instance)),
        },
        Err(_) => NewRes {
            content: Err(String::from("Failed to aquire lock")),
        },
    }
}

#[fmt_function]
pub fn handle_is_valid(req: IsValidReq) -> IsValidRes {
    match global_state::GLOBAL_STATE.lock() {
        Ok(x) => IsValidRes {
            content: Ok(x.instance_exists(&req.key)),
        },
        Err(_) => IsValidRes {
            content: Err(String::from("Failed to aquire lock")),
        },
    }
}

#[fmt_function]
pub fn handle_remove(req: RemoveReq) -> RemoveRes {
    match global_state::GLOBAL_STATE.lock() {
        Ok(x) => RemoveRes {
            content: Ok(x.instance_remove(&req.key)),
        },
        Err(_) => RemoveRes {
            content: Err(String::from("Failed to aquire lock")),
        },
    }
}

#[fmt_function]
pub fn handle_invoke(req: InvokeReq) -> InvokeRes {
    let key = &req.key;

    match global_state::GLOBAL_STATE.lock() {
        Ok(x) => {
            if !x.instance_exists(key) {
                return InvokeRes {
                    content: Err(String::from("Instance does not exist")),
                };
            }

            if let Some(inst) = x.instance(key) {
                let content = match inst.call_entry_point() {
                    Ok(code) => Ok(code),
                    Err(e) => Err(format!("Error while invoking instance: {}", e)),
                };

                InvokeRes { content }
            } else {
                InvokeRes {
                    content: Err(String::from("Failed to retrieve instance")),
                }
            }
        }
        Err(_) => InvokeRes {
            content: Err(String::from("Failed to aquire lock")),
        },
    }
}
