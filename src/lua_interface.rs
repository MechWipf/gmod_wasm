use gmod_sys::{
    lua_State,
    lua_wrapper::{LuaBase, SpecialType},
};

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn gmod13_open(state: *mut lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*state).luabase });
    unsafe { lua.set_state(state) };

    lua.push_special(SpecialType::GlobalTable);
    lua.push_c_function(None);
    lua.set_field(-2, "wasmer");
    lua.pop(1);

    lua.print("Web Assembly library loaded!");

    0
}

#[no_mangle]
pub extern "C" fn gmod13_close(_state: *mut lua_State) -> i32 {
    0
}
