use std::borrow::Cow;

use wasmer::{imports, wat2wasm, Cranelift, Function, Instance, Module, Store, JIT};

use crate::LuaWasmerError;

pub fn wasm_instance_bytes(wasm_bytes: Cow<[u8]>) -> Result<Box<&Function>, LuaWasmerError> {
    Ok(wasm_instance(wasm_bytes)?)
}

pub fn wasm_instance_str(wat: &str) -> Result<Box<&Function>, LuaWasmerError> {
    let wasm_bytes = match wat2wasm(wat.as_bytes()) {
        Ok(b) => b,
        _ => return Err(LuaWasmerError::Wat2Wasm),
    };

    Ok(wasm_instance(wasm_bytes)?)
}

fn wasm_instance(wasm_bytes: Cow<[u8]>) -> Result<Box<&Function>, LuaWasmerError> {
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
        Err(e) => return Err(LuaWasmerError::InstanceError(e)),
    };

    let main = match instance.exports.get_function("main") {
        Ok(x) => x,
        Err(e) => return Err(LuaWasmerError::NoMainMethodFound(e)),
    };

    Ok(Box::new(main))
}

/*
    let results = match main.call(&[]) {
        Ok(x) => x,
        Err(e) => return Err(LuaWasmerError::RuntimeError(e)),
    };
*/
