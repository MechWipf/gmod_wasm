use lrpc::*;
use lua_wasmer_common::rpc::{EchoReq, EchoRes, InvokeReq, InvokeRes, NewFromWatReq, NewRes};

fn main() {
    let mut conn = Connection::new("[::1]:15081");
    let resp: lrpc::Result<EchoRes> = conn.invoke(fun!(
        "echo",
        EchoReq {
            msg: "Hello World!".to_string(),
        }
    ));

    print!("{:#?}", resp);

    let resp: lrpc::Result<NewRes> = conn.invoke(fun!(
        "new_from_wat",
        NewFromWatReq {
            wat: String::from(r#"(module (type $main_t (func (param i32) (result i32))) (func $main_f (type $main_t) (param i32) (result i32) (i32.const 42)) (export "main" (func $main_f)))"#),
        }
    ));

    print!("{:#?}", resp);
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert!(resp.content.is_ok());

    let key = resp.content.unwrap();

    let resp: lrpc::Result<InvokeRes> = conn.invoke(fun!("invoke", InvokeReq { key }));

    print!("{:#?}", resp);
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert!(resp.content.is_ok());
}
