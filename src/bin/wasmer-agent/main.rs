use fibers::{Executor, Spawn, ThreadPoolExecutor};
use fibers_rpc::{
    client::ClientService, client::ClientServiceBuilder, server::HandleCall, server::Reply,
    server::ServerBuilder, Call, ProcedureId,
};
use futures::Future;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod error;
mod global_state;
mod wasm;
mod wasm_service;

fn main() {
    let mut executor = ThreadPoolExecutor::new().unwrap();

    let srv_addr = "[::1]:15081".parse().unwrap();

    let mut builder = ServerBuilder::new(srv_addr);
    builder.add_call_handler(wasm_service::WasmHandler);
    let server = builder.finish(executor.handle());

    let mon = executor.spawn_monitor(server);
    let whatever = executor.run_fiber(mon);

    println!("{:?}", whatever);
}

mod test {
    use crate::wasm_service::HelloRpc;
    use fibers::Executor;
    use fibers::Spawn;
    use fibers::ThreadPoolExecutor;
    use fibers_rpc::client::CallClient;
    use fibers_rpc::client::ClientServiceBuilder;
    use fibers_rpc::Call;
    use fibers_rpc::Cast;

    #[test]
    fn does_this_run() {
        let mut executor = ThreadPoolExecutor::new().unwrap();
        let srv_addr = "[::1]:15081".parse().unwrap();

        let service = ClientServiceBuilder::new().finish(executor.handle());
        let service_handle = service.handle();

        let request = Vec::from(&b"Hello"[..]);
        let response = HelloRpc::client(&service_handle).call(srv_addr, request);
        let response = executor.spawn_monitor(response);
        let response = executor.run_future(response);

        println!("{:?}", response);
    }
}
