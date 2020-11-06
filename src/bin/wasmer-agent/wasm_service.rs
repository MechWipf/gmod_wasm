use bytecodec::bytes::{BytesEncoder, RemainingBytesDecoder};
use fibers_rpc::{
    server::{HandleCall, Reply},
    Call, ProcedureId,
};

pub(crate) struct HelloRpc;

impl Call for HelloRpc {
    const ID: ProcedureId = ProcedureId(0);
    const NAME: &'static str = "wasm";

    type Req = Vec<u8>;
    type ReqEncoder = BytesEncoder<Vec<u8>>;
    type ReqDecoder = RemainingBytesDecoder;

    type Res = Vec<u8>;
    type ResEncoder = BytesEncoder<Vec<u8>>;
    type ResDecoder = RemainingBytesDecoder;
}

pub(crate) struct WasmHandler;

impl HandleCall<HelloRpc> for WasmHandler {
    fn handle_call(&self, request: <HelloRpc as Call>::Req) -> Reply<HelloRpc> {
        Reply::done(request)
    }
}
