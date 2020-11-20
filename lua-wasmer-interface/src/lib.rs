use gmod_sys::lua_wrapper::{LuaBase, SpecialType};

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn gmod13_open(state: *mut gmod_sys::lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*state).luabase });
    unsafe { lua.set_state(state) };
    lua.push_special(SpecialType::GlobalTable);
    lua.create_table();
    lua.push_c_function(Some(new_from_wat));
    lua.set_field(-2, "newFromWat");
    lua.push_c_function(Some(run));
    lua.set_field(-2, "run");
    lua.set_field(-2, "WasmInterface");
    lua.pop(1);

    lua.print("Web Assembly library loaded!");

    0
}

#[no_mangle]
pub extern "C" fn gmod13_close(_state: *mut gmod_sys::lua_State) -> i32 {
    0
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn new_from_wat(l: *mut gmod_sys::lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*l).luabase });

    match handler::handle_new_from_wat(&lua) {
        Ok(x) => x,
        Err(e) => {
            lua.throw_error(&format!("{}", e));
            0
        }
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn run(l: *mut gmod_sys::lua_State) -> i32 {
    let lua = LuaBase::new(unsafe { (*l).luabase });

    match handler::handle_run(&lua) {
        Ok(x) => x,
        Err(e) => {
            lua.throw_error(&format!("{}", e));
            0
        }
    }
}

mod handler {
    use anyhow::bail;
    use gmod_sys::lua_wrapper::LuaBase;
    use lrpc::{fun, Connection, Store};
    use lua_wasmer_common::rpc::{
        InvokeReq, InvokeRes, NewFromWatReq, NewRes, CALL_INVOKE, CALL_NEW_FROM_WAT,
    };

    fn get_connection() -> Connection {
        Connection::new("[::1]:15081")
    }

    fn invoke_new_from_wat(conn: &mut Connection, req: NewFromWatReq) -> lrpc::Result<NewRes> {
        conn.invoke(fun!(CALL_NEW_FROM_WAT, req))
    }

    fn invoke_invoke(conn: &mut Connection, req: InvokeReq) -> lrpc::Result<InvokeRes> {
        conn.invoke(fun!(CALL_INVOKE, req))
    }

    pub(crate) fn handle_new_from_wat(l: &LuaBase) -> anyhow::Result<i32> {
        l.check_type(1, gmod_sys::lua_wrapper::Type::String);
        let wat = match l.get_string(1) {
            Some(x) => x,
            _ => bail!("Argument error: Invalid string"),
        };

        let mut conn = get_connection();
        let resp = match invoke_new_from_wat(&mut conn, NewFromWatReq { wat }) {
            Ok(x) => match x.content {
                Ok(x) => x,
                Err(e) => bail!(e),
            },
            Err(e) => bail!(e),
        };

        l.push_string(&resp);
        Ok(1)
    }

    pub(crate) fn handle_run(l: &LuaBase) -> anyhow::Result<i32> {
        l.check_type(1, gmod_sys::lua_wrapper::Type::String);
        let key = match l.get_string(1) {
            Some(x) => x,
            _ => bail!("Argument error: Invalid string"),
        };

        let mut conn = get_connection();
        let resp = match invoke_invoke(&mut conn, InvokeReq { key }) {
            Ok(x) => match x.content {
                Ok(x) => x,
                Err(e) => bail!(e),
            },
            Err(e) => bail!(e),
        };

        l.push_number(resp as f64);
        Ok(1)
    }
}
