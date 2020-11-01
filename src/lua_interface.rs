use crate::{
    global_state::{drop_global_state, init_global_state},
    wasm, LuaWasmerError,
};
use gmod_sys::{
    lua_State,
    lua_wrapper::{LuaBase, SpecialType},
};

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

fn handle_wasmer(l: &LuaBase) -> Result<i32, LuaWasmerError> {
    l.check_type(1, gmod_sys::lua_wrapper::Type::String);
    let str_val = match l.get_string(1) {
        Some(s) => s,
        _ => return Err(LuaWasmerError::InvalidString),
    };

    let instance = match wasm::wasm_instance_str(&str_val) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let result = match instance.call_entry_point() {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    l.push_number(result as f64);
    Ok(1)
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn example1(l: *mut lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*l).luabase });

    1
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn example2(l: *mut lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*l).luabase });

    0
}

#[derive(Debug)]
pub struct UdTest {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn gmod13_open(state: *mut lua_State) -> i32 {
    // Normaly I would create this object in main and make it accessible over parameters
    init_global_state();

    let lua = LuaBase::new(unsafe { (*state).luabase });
    unsafe { lua.set_state(state) };
    lua.push_special(SpecialType::GlobalTable);
    lua.push_c_function(Some(example1));
    lua.set_field(-2, "example1");
    lua.push_c_function(Some(example2));
    lua.set_field(-2, "example2");
    lua.push_c_function(Some(wasmer));
    lua.set_field(-2, "wasmer");
    lua.pop(1);

    lua.print("Web Assembly library loaded!");

    0
}

#[no_mangle]
pub extern "C" fn gmod13_close(_state: *mut lua_State) -> i32 {
    // This might freeze the world
    drop_global_state();

    0
}
