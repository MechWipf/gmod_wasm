#ifndef WRAPPER_H
#define WRAPPER_H
#define GMOD_ALLOW_DEPRECATED

#include <GarrysMod/Lua/Interface.h>
#include <GarrysMod/Lua/LuaBase.h>
#include <GarrysMod/Lua/SourceCompat.h>
#include <GarrysMod/Lua/Types.h>

extern "C"
{
    struct wrap_str
    {
        const char *str_ptr;
        unsigned int str_len;
    };

    int lua_top(GarrysMod::Lua::ILuaBase *lua);

    void lua_push(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    void lua_pop(GarrysMod::Lua::ILuaBase *lua, int iAmt);

    void lua_get_table(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    void lua_get_field(GarrysMod::Lua::ILuaBase *lua, int iStackPos, const char *strName);

    void lua_set_field(GarrysMod::Lua::ILuaBase *lua, int iStackPos, const char *strName);

    void lua_create_table(GarrysMod::Lua::ILuaBase *lua);

    void lua_set_table(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    void lua_set_meta_table(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    bool lua_get_meta_table(GarrysMod::Lua::ILuaBase *lua, int i);

    void lua_call(GarrysMod::Lua::ILuaBase *lua, int iArgs, int iResults);

    int lua_p_call(GarrysMod::Lua::ILuaBase *lua, int iArgs, int iResults, int iErrorFunc);

    int lua_equal(GarrysMod::Lua::ILuaBase *lua, int iA, int iB);

    int lua_raw_equal(GarrysMod::Lua::ILuaBase *lua, int iA, int iB);

    void lua_insert(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    void lua_remove(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    int lua_next(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    void lua_throw_error(GarrysMod::Lua::ILuaBase *lua, const char *strError);

    void lua_check_type(GarrysMod::Lua::ILuaBase *lua, int iStackPos, int iType);

    void lua_arg_error(GarrysMod::Lua::ILuaBase *lua, int iArgNum, const char *strMessage);

    void lua_raw_get(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    void lua_raw_set(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    wrap_str lua_get_string(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    double lua_get_number(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    bool lua_get_bool(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    GarrysMod::Lua::CFunc lua_get_c_func(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    void lua_push_nil(GarrysMod::Lua::ILuaBase *lua);

    void lua_push_string(GarrysMod::Lua::ILuaBase *lua, const char *val, unsigned int iLen);

    void lua_push_number(GarrysMod::Lua::ILuaBase *lua, double val);

    void lua_push_bool(GarrysMod::Lua::ILuaBase *lua, bool val);

    void lua_push_c_function(GarrysMod::Lua::ILuaBase *lua, GarrysMod::Lua::CFunc val);

    void lua_push_c_closure(GarrysMod::Lua::ILuaBase *lua, GarrysMod::Lua::CFunc val, int iVars);

    int lua_reference_create(GarrysMod::Lua::ILuaBase *lua);

    void lua_reference_free(GarrysMod::Lua::ILuaBase *lua, int i);

    void lua_reference_push(GarrysMod::Lua::ILuaBase *lua, int i);

    void lua_push_special(GarrysMod::Lua::ILuaBase *lua, int iType);

    bool lua_is_type(GarrysMod::Lua::ILuaBase *lua, int iStackPos, int iType);

    int lua_get_type(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    const char *lua_get_type_name(GarrysMod::Lua::ILuaBase *lua, int iType);

    const char *lua_check_string(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    double lua_check_number(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    int lua_obj_len(GarrysMod::Lua::ILuaBase *lua, int iStackPos);

    // const QAngle& lua_get_angle( GarrysMod::Lua::ILuaBase* lua, int iStackPos = -1 );

    // const Vector& lua_get_vector( GarrysMod::Lua::ILuaBase* lua, int iStackPos = -1 );

    // void lua_push_angle( GarrysMod::Lua::ILuaBase* lua, const QAngle& val );

    // void lua_push_vector( GarrysMod::Lua::ILuaBase* lua, const Vector& val );

    void lua_set_state(GarrysMod::Lua::ILuaBase *lua, lua_State *L);

    int lua_create_meta_table(GarrysMod::Lua::ILuaBase *lua, const char *strName);

    bool lua_push_meta_table(GarrysMod::Lua::ILuaBase *lua, int iType);

    void lua_push_usertype(GarrysMod::Lua::ILuaBase *lua, void *data, int iType);

    void lua_set_usertype(GarrysMod::Lua::ILuaBase *lua, int iStackPos, void *data);

    void *lua_get_usertype(GarrysMod::Lua::ILuaBase *lua, int iStackPos, int iType);
}

#endif