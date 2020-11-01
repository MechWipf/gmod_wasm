#include "wrapper.hpp"
#include <GarrysMod/Lua/Interface.h>
#include <GarrysMod/Lua/LuaBase.h>
#include <GarrysMod/Lua/SourceCompat.h>
#include <string>
#include <cstring>

using namespace std;
using namespace GarrysMod;
using namespace GarrysMod::Lua;

/* extern "C" */ int lua_top(GarrysMod::Lua::ILuaBase *lua)
{
    return lua->Top();
}

/* extern "C" */ void lua_push(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->Push(iStackPos);
}

/* extern "C" */ void lua_pop(GarrysMod::Lua::ILuaBase *lua, int iAmt = 1)
{
    lua->Pop(iAmt);
}

/* extern "C" */ void lua_get_table(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->GetTable(iStackPos);
}

/* extern "C" */ void lua_get_field(GarrysMod::Lua::ILuaBase *lua, int iStackPos, const char *strName)
{
    lua->GetField(iStackPos, strName);
}

/* extern "C" */ void lua_set_field(GarrysMod::Lua::ILuaBase *lua, int iStackPos, const char *strName)
{
    lua->SetField(iStackPos, strName);
}

/* extern "C" */ void lua_create_table(GarrysMod::Lua::ILuaBase *lua)
{
    lua->CreateTable();
}

/* extern "C" */ void lua_set_table(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->SetTable(iStackPos);
}

/* extern "C" */ void lua_set_meta_table(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->SetMetaTable(iStackPos);
}

/* extern "C" */ bool lua_get_meta_table(GarrysMod::Lua::ILuaBase *lua, int i)
{
    return lua->GetMetaTable(i);
}

/* extern "C" */ void lua_call(GarrysMod::Lua::ILuaBase *lua, int iArgs, int iResults)
{
    lua->Call(iArgs, iResults);
}

/* extern "C" */ int lua_p_call(GarrysMod::Lua::ILuaBase *lua, int iArgs, int iResults, int iErrorFunc)
{
    return lua->PCall(iArgs, iResults, iErrorFunc);
}

/* extern "C" */ int lua_equal(GarrysMod::Lua::ILuaBase *lua, int iA, int iB)
{
    return lua->Equal(iA, iB);
}

/* extern "C" */ int lua_raw_equal(GarrysMod::Lua::ILuaBase *lua, int iA, int iB)
{
    return lua->RawEqual(iA, iB);
}

/* extern "C" */ void lua_insert(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->Insert(iStackPos);
}

/* extern "C" */ void lua_remove(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->Remove(iStackPos);
}

/* extern "C" */ int lua_next(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    return lua->Next(iStackPos);
}

/* extern "C" */ void lua_throw_error(GarrysMod::Lua::ILuaBase *lua, const char *strError)
{
    lua->ThrowError(strError);
}

/* extern "C" */ void lua_check_type(GarrysMod::Lua::ILuaBase *lua, int iStackPos, int iType)
{
    lua->CheckType(iStackPos, iType);
}

/* extern "C" */ void lua_arg_error(GarrysMod::Lua::ILuaBase *lua, int iArgNum, const char *strMessage)
{
    lua->ArgError(iArgNum, strMessage);
}

/* extern "C" */ void lua_raw_get(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->RawGet(iStackPos);
}

/* extern "C" */ void lua_raw_set(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    lua->RawSet(iStackPos);
}

/* extern "C" */ wrap_str lua_get_string(GarrysMod::Lua::ILuaBase *lua, int iStackPos = -1)
{
    wrap_str s;
    s.str_len = 0;
    s.str_ptr = lua->GetString(iStackPos, &s.str_len);
    return s;
}

/* extern "C" */ double lua_get_number(GarrysMod::Lua::ILuaBase *lua, int iStackPos = -1)
{
    return lua->GetNumber(iStackPos);
}

/* extern "C" */ bool lua_get_bool(GarrysMod::Lua::ILuaBase *lua, int iStackPos = -1)
{
    return lua->GetBool(iStackPos);
}

/* extern "C" */ CFunc lua_get_c_func(GarrysMod::Lua::ILuaBase *lua, int iStackPos = -1)
{
    return lua->GetCFunction(iStackPos);
}

/* extern "C" */ void lua_push_nil(GarrysMod::Lua::ILuaBase *lua)
{
    lua->PushNil();
}

/* extern "C" */ void lua_push_string(GarrysMod::Lua::ILuaBase *lua, const char *val, unsigned int iLen = 0)
{
    lua->PushString(val, iLen);
}

/* extern "C" */ void lua_push_number(GarrysMod::Lua::ILuaBase *lua, double val)
{
    lua->PushNumber(val);
}

/* extern "C" */ void lua_push_bool(GarrysMod::Lua::ILuaBase *lua, bool val)
{
    lua->PushBool(val);
}

/* extern "C" */ void lua_push_c_function(GarrysMod::Lua::ILuaBase *lua, CFunc val)
{
    lua->PushCFunction(val);
}

/* extern "C" */ void lua_push_c_closure(GarrysMod::Lua::ILuaBase *lua, CFunc val, int iVars)
{
    lua->PushCClosure(val, iVars);
}

/* extern "C" */ int lua_reference_create(GarrysMod::Lua::ILuaBase *lua)
{
    return lua->ReferenceCreate();
}

/* extern "C" */ void lua_reference_free(GarrysMod::Lua::ILuaBase *lua, int i)
{
    lua->ReferenceFree(i);
}

/* extern "C" */ void lua_reference_push(GarrysMod::Lua::ILuaBase *lua, int i)
{
    lua->ReferencePush(i);
}

/* extern "C" */ void lua_push_special(GarrysMod::Lua::ILuaBase *lua, int iType)
{
    lua->PushSpecial(iType);
}

/* extern "C" */ bool lua_is_type(GarrysMod::Lua::ILuaBase *lua, int iStackPos, int iType)
{
    return lua->IsType(iStackPos, iType);
}

/* extern "C" */ int lua_get_type(GarrysMod::Lua::ILuaBase *lua, int iStackPos)
{
    return lua->GetType(iStackPos);
}

/* extern "C" */ const char *lua_get_type_name(GarrysMod::Lua::ILuaBase *lua, int iType)
{
    return lua->GetTypeName(iType);
}

/* extern "C" */ const char *lua_check_string(GarrysMod::Lua::ILuaBase *lua, int iStackPos = -1)
{
    return lua->CheckString(iStackPos);
}

/* extern "C" */ double lua_check_number(GarrysMod::Lua::ILuaBase *lua, int iStackPos = -1)
{
    return lua->CheckNumber(iStackPos);
}

/* extern "C" */ int lua_obj_len(GarrysMod::Lua::ILuaBase *lua, int iStackPos = -1)
{
    return lua->ObjLen(iStackPos);
}

// /* extern "C" */ const QAngle& lua_get_angle( GarrysMod::Lua::ILuaBase* lua, int iStackPos = -1 )
// {
//     return lua->GetAngle(iStackPos);
// }

// /* extern "C" */ const Vector& lua_get_vector( GarrysMod::Lua::ILuaBase* lua, int iStackPos = -1 )
// {
//     return lua->GetVector(iStackPos);
// }

// /* extern "C" */ void lua_push_angle( GarrysMod::Lua::ILuaBase* lua, const QAngle& val )
// {
//     lua->PushAngle(val);
// }

// /* extern "C" */ void lua_push_vector( GarrysMod::Lua::ILuaBase* lua, const Vector& val )
// {
//     lua->PushVector(val);
// }

/* extern "C" */ void lua_set_state(GarrysMod::Lua::ILuaBase *lua, lua_State *L)
{
    lua->SetState(L);
}

/* extern "C" */ int lua_create_meta_table(GarrysMod::Lua::ILuaBase *lua, const char *strName)
{
    return lua->CreateMetaTable(strName);
}

/* extern "C" */ bool lua_push_meta_table(GarrysMod::Lua::ILuaBase *lua, int iType)
{
    return lua->PushMetaTable(iType);
}

/* extern "C" */ void lua_set_user_type(GarrysMod::Lua::ILuaBase *lua, int iStackPos, void *data)
{
    lua->SetUserType(iStackPos, data);
}

/* extern "C" */ void lua_push_usertype(GarrysMod::Lua::ILuaBase *lua, void *data, int iType)
{
    lua->PushUserType(data, iType);
}

/* extern "C" */ void *lua_get_usertype(GarrysMod::Lua::ILuaBase *lua, int iStackPos, int iType)
{
    return lua->GetUserdata(iStackPos);
}