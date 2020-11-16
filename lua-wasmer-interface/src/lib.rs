use gmod_sys::{
    lua_State,
    lua_wrapper::{LuaBase, SpecialType},
};

use thiserror::Error;

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

fn handle_wasmer(l: &LuaBase) -> Result<i32, LuaInterfaceError> {
    l.check_type(1, gmod_sys::lua_wrapper::Type::String);
    let str_val = match l.get_string(1) {
        Some(s) => s,
        _ => return Err(LuaInterfaceError::InvalidString),
    };

    l.push_number(0 as f64);
    Ok(1)
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn gmod13_open(state: *mut lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*state).luabase });
    unsafe { lua.set_state(state) };
    lua.push_special(SpecialType::GlobalTable);
    lua.create_table();
    lua.push_c_function(Some(wasmer));
    lua.set_field(-2, "wasmer");
    lua.set_field(-2, "wasm_interface");
    lua.pop(1);

    lua.print("Web Assembly library loaded!");

    0
}

#[no_mangle]
pub extern "C" fn gmod13_close(_state: *mut lua_State) -> i32 {
    1
}

#[derive(Error, Debug)]
pub enum LuaInterfaceError {
    #[error("Invalid string")]
    InvalidString,
}
