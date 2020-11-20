use lrpc::*;

pub const CALL_NEW_FROM_WAT: &str = "new_from_wat";
pub const CALL_NEW_FROM_BINARY: &str = "new_from_binary";
pub const CALL_INVOKE: &str = "invoke";
pub const CALL_IS_VALID: &str = "is_valid";
pub const CALL_REMOVE: &str = "remove";

#[derive(CommonStore, Debug)]
pub struct EchoReq {
    pub msg: String,
}

#[derive(CommonStore, Debug)]
pub struct EchoRes {
    pub content: Result<String>,
}

#[derive(CommonStore, Debug)]
pub struct NewFromWatReq {
    pub wat: String,
}

#[derive(CommonStore, Debug)]
pub struct NewFromBinaryReq {
    pub binary: Vec<u8>,
}

#[derive(CommonStore, Debug)]
pub struct NewRes {
    pub content: Result<String>,
}

#[derive(CommonStore, Debug)]
pub struct IsValidReq {
    pub key: String,
}

#[derive(CommonStore, Debug)]
pub struct IsValidRes {
    pub content: Result<bool>,
}

#[derive(CommonStore, Debug)]
pub struct RemoveReq {
    pub key: String,
}

#[derive(CommonStore, Debug)]
pub struct RemoveRes {
    pub content: Result<bool>,
}

#[derive(CommonStore, Debug)]
pub struct InvokeReq {
    pub key: String,
}

#[derive(CommonStore, Debug)]
pub struct InvokeRes {
    pub content: Result<i32>,
}
