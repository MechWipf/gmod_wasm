use fibers::{Executor, Spawn, ThreadPoolExecutor};
use lrpc::{self, service, Fun};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = ThreadPoolExecutor::new().unwrap();

    executor.spawn_fn(move || {
        let mut srv_fun = Fun::new();

        srv_fun.regist("echo", wasm_server::handle_echo);
        srv_fun.regist("new_from_wat", wasm_server::handle_new_from_wat);
        srv_fun.regist("new_from_binary", wasm_server::handle_new_from_binary);
        srv_fun.regist("is_valid", wasm_server::handle_is_valid);
        srv_fun.regist("remove", wasm_server::handle_remove);
        srv_fun.regist("invoke", wasm_server::handle_invoke);

        service(srv_fun, "[::1]:15081");
        Ok(())
    });

    executor.run().map_err(|e| panic! {"{}", e})
}
