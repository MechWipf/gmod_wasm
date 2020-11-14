use lrpc::*;

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
