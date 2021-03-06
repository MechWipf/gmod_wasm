use lua_wasmer_common::err::LuaWasmerError;
use std::sync::Mutex;
use wasmer::{imports, wat2wasm, Cranelift, Instance, Module, NativeFunc, Store, JIT};

pub fn wasm_instance_bytes(wasm_bytes: impl AsRef<[u8]>) -> Result<WasmInstance, LuaWasmerError> {
    Ok(WasmInstance::new(wasm_bytes)?)
}

pub fn wasm_instance_str(wat: &str) -> Result<WasmInstance, LuaWasmerError> {
    let wasm_bytes = match wat2wasm(wat.as_bytes()) {
        Ok(b) => b,
        _ => return Err(LuaWasmerError::Wat2Wasm),
    };

    Ok(WasmInstance::new(wasm_bytes)?)
}

#[derive(Debug)]
pub struct WasmInstance {
    instance: Mutex<Instance>,
}

impl WasmInstance {
    fn new(wasm_bytes: impl AsRef<[u8]>) -> Result<Self, LuaWasmerError> {
        let compiler = Cranelift::new();

        let store = Store::new(&JIT::new(&compiler).engine());

        // Here compile
        let module = match Module::new(&store, wasm_bytes) {
            Ok(m) => m,
            Err(e) => {
                return Err(LuaWasmerError::CompileError {
                    message: format!("{:?}", e),
                })
            }
        };

        let import_object = imports! {};

        // Instantiate module
        let instance = match Instance::new(&module, &import_object) {
            Ok(i) => i,
            Err(e) => {
                return Err(LuaWasmerError::InstanceError {
                    message: e.to_string(),
                })
            }
        };

        Ok(Self {
            instance: Mutex::new(instance),
        })
    }

    pub fn call_entry_point(&self) -> Result<i32, LuaWasmerError> {
        let inst = self.instance.lock().expect("Should work?");
        let entry_point: NativeFunc<i32, i32> = match inst.exports.get_native_function("main") {
            Ok(x) => x,
            Err(e) => {
                return Err(LuaWasmerError::NoMainMethodFound {
                    message: e.to_string(),
                })
            }
        };

        let result = match entry_point.call(0) {
            Ok(x) => x,
            Err(e) => {
                return Err(LuaWasmerError::RuntimeError {
                    message: e.to_string(),
                })
            }
        };

        Ok(result)
    }
}

#[test]
fn wasmer_basic() {
    let code = r#"(module (type $main_t (func (param i32) (result i32))) (func $main_f (type $main_t) (param i32) (result i32) (i32.const 42)) (export "main" (func $main_f)))"#;
    let inst = wasm_instance_str(code).unwrap();
    let result_code = inst.call_entry_point().unwrap();

    assert_eq!(result_code, 42);
}
