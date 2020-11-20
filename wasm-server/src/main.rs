use fibers::{Executor, Spawn, ThreadPoolExecutor};
use lrpc::{self, service, Fun};
use lua_wasmer_common::rpc::{
    CALL_INVOKE, CALL_IS_VALID, CALL_NEW_FROM_BINARY, CALL_NEW_FROM_WAT, CALL_REMOVE,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = ThreadPoolExecutor::new().unwrap();

    executor.spawn_fn(move || {
        let mut srv_fun = Fun::new();

        srv_fun.regist("echo", wasm_server::handle_echo);
        srv_fun.regist(CALL_NEW_FROM_WAT, wasm_server::handle_new_from_wat);
        srv_fun.regist(CALL_NEW_FROM_BINARY, wasm_server::handle_new_from_binary);
        srv_fun.regist(CALL_IS_VALID, wasm_server::handle_is_valid);
        srv_fun.regist(CALL_REMOVE, wasm_server::handle_remove);
        srv_fun.regist(CALL_INVOKE, wasm_server::handle_invoke);

        service(srv_fun, "[::1]:15081");
        Ok(())
    });

    executor.run().map_err(|e| panic! {"{}", e})
}
