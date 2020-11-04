use futures::{future::ready, future::Ready};
use std::borrow::Cow;
use tarpc::context;

#[tarpc::service]
trait WasmService {
    async fn from_wat(wat: String) -> String;
    async fn from_binary(module: Cow<'static, [u8]>) -> String;
}

#[derive(Clone)]
pub struct WasmServer;

impl WasmService for WasmServer {
    type FromWatFut = Ready<String>;
    type FromBinaryFut = Ready<String>;

    fn from_wat(self, _: context::Context, _wat: String) -> Self::FromWatFut {
        ready("Cake".to_string())
    }

    fn from_binary(self, _: context::Context, _module: Cow<'static, [u8]>) -> Self::FromBinaryFut {
        ready("Cake".to_string())
    }
}
