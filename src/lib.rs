mod wasm;

use gmod_sys::{
    lua_State,
    lua_wrapper::{LuaBase, SpecialType},
};

use thiserror::Error;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn example(l: *mut lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*l).luabase });

    if let Some(str_val) = lua.get_string(1) {
        lua.create_table();
        lua.push_string(&str_val);
        lua.set_field(-2, "value");
        1
    } else {
        lua.throw_error("Parameter invalid.");
        0
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wasmer(l: *mut lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*l).luabase });

    match handle_wasmer(&lua) {
        Ok(x) => x,
        Err(e) => {
            lua.throw_error(&format!("{:?}", e));
            0
        }
    }
}

// lua_run wasmer '(module (type $main_t (func (result i32))) (func $main_f (type $main_t) (result i32) (i32.const 42)) (export "main" (func $main_f)))'
fn handle_wasmer(l: &LuaBase) -> Result<i32, LuaWasmerError> {
    l.check_type(1, gmod_sys::lua_wrapper::Type::String);
    let str = match l.get_string(1) {
        Some(s) => s,
        _ => return Err(LuaWasmerError::InvalidString),
    };

    l.push_string(&format!("Result: {:?}", results));
    Ok(1)
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn gmod13_open(state: *mut lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*state).luabase });
    unsafe { lua.set_state(state) };
    lua.push_special(SpecialType::GlobalTable);
    lua.push_c_function(Some(example));
    lua.set_field(-2, "example");
    lua.push_c_function(Some(wasmer));
    lua.set_field(-2, "wasmer");
    lua.pop(1);

    lua.print("Rust library loaded!");

    0
}

#[no_mangle]
pub extern "C" fn gmod13_close(_state: *mut lua_State) -> i32 {
    1
}

#[derive(Error, Debug)]
pub enum LuaWasmerError {
    #[error("Invalid string")]
    InvalidString,
    #[error("Unable to convert wat to wasm")]
    Wat2Wasm,
    #[error("Compile error {message:?}")]
    CompileError { message: String },
    #[error("Unable to create instance")]
    InstanceError(#[from] wasmer::InstantiationError),
    #[error("Did you specify a main method?")]
    NoMainMethodFound(#[from] wasmer::ExportError),
    #[error("World is broken")]
    RuntimeError(#[from] wasmer::RuntimeError),
}
