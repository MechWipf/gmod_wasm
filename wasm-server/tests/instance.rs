use lrpc::*;
use lua_wasmer_common::rpc::{
    InvokeReq, InvokeRes, IsValidReq, IsValidRes, NewFromWatReq, NewRes, RemoveReq, RemoveRes,
};
use std::{panic, process::Command};

const WAT_SIMPLE: &str = r#"(module (type $main_t (func (param i32) (result i32))) (func $main_f (type $main_t) (param i32) (result i32) (i32.const 42)) (export "main" (func $main_f)))"#;
const WAT_ERR: &str = r#"(module (type $main_t asdasd (func (param i32) (result i32))) (func $main_f (type $main_t) (param i32) (result i32) (i32.const 42)) (export "main" (func $main_f)))"#;

pub fn run_test<T>(test: T)
where
    T: FnOnce() + panic::UnwindSafe,
{
    let mut server = Command::new("target/Debug/gmod_wasm.exe").spawn().unwrap();

    let result = panic::catch_unwind(|| test());

    server.kill().unwrap();

    if let Err(e) = result {
        panic::resume_unwind(e);
    }
}

#[test]
fn test_new_instance_from_wat_ok() {
    run_test(|| {
        let mut conn = Connection::new("[::1]:15081");

        let resp: lrpc::Result<NewRes> = conn.invoke(fun!(
            "new_from_wat",
            NewFromWatReq {
                wat: String::from(WAT_SIMPLE),
            }
        ));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert!(resp.content.is_ok());
    })
}

#[test]
fn test_new_instance_from_wat_fail() {
    run_test(|| {
        let mut conn = Connection::new("[::1]:15081");

        let resp: lrpc::Result<NewRes> = conn.invoke(fun!(
            "new_from_wat",
            NewFromWatReq {
                wat: String::from(WAT_ERR),
            }
        ));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert!(resp.content.is_err());
    })
}

#[test]
fn test_instance_is_valid() {
    run_test(|| {
        let mut conn = Connection::new("[::1]:15081");

        let resp: lrpc::Result<NewRes> = conn.invoke(fun!(
            "new_from_wat",
            NewFromWatReq {
                wat: String::from(WAT_SIMPLE),
            }
        ));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert!(resp.content.is_ok());

        let key = resp.content.unwrap();

        let resp: lrpc::Result<IsValidRes> = conn.invoke(fun!("is_valid", IsValidReq { key }));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.content, Ok(true));
    })
}

#[test]
fn test_instance_is_valid_fail() {
    run_test(|| {
        let mut conn = Connection::new("[::1]:15081");

        let key = String::from("does-not-exists");

        let resp: lrpc::Result<IsValidRes> = conn.invoke(fun!("is_valid", IsValidReq { key }));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.content, Ok(false));
    })
}

#[test]
fn test_instance_remove() {
    run_test(|| {
        let mut conn = Connection::new("[::1]:15081");

        let resp: lrpc::Result<NewRes> = conn.invoke(fun!(
            "new_from_wat",
            NewFromWatReq {
                wat: String::from(WAT_SIMPLE),
            }
        ));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert!(resp.content.is_ok());

        let key = resp.content.unwrap();
        let resp: lrpc::Result<IsValidRes> =
            conn.invoke(fun!("is_valid", IsValidReq { key: key.clone() }));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.content, Ok(true));

        let resp: lrpc::Result<RemoveRes> = conn.invoke(fun!("is_valid", RemoveReq { key }));

        print!("{:#?}", resp);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.content, Ok(true));
    })
}

#[test]
fn test_invoke_simple() {
    run_test(|| {
        let mut conn = Connection::new("[::1]:15081");

        let resp: lrpc::Result<NewRes> = conn.invoke(fun!(
            "new_from_wat",
            NewFromWatReq {
                wat: String::from(WAT_SIMPLE),
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
        assert_eq!(resp.content, Ok(42));
    })
}
