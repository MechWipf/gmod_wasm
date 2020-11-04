use futures::{future::ready, stream, Stream};
use tarpc::{server, transport};
use wasm_service::WasmServer;

mod error;
mod global_state;
mod wasm;
mod wasm_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //todo implement server

    Ok(())
}
