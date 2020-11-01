use super::generated::{
    lua_State, lua_arg_error, lua_call, lua_check_number, lua_check_string, lua_check_type,
    lua_create_table, lua_equal, lua_get_bool, lua_get_c_func, lua_get_field, lua_get_meta_table,
    lua_get_number, lua_get_string, lua_get_table, lua_get_type, lua_get_type_name,
    lua_get_usertype, lua_insert, lua_is_type, lua_next, lua_obj_len, lua_p_call, lua_pop,
    lua_push, lua_push_bool, lua_push_c_closure, lua_push_c_function, lua_push_nil,
    lua_push_number, lua_push_special, lua_push_string, lua_push_usertype, lua_raw_equal,
    lua_raw_get, lua_raw_set, lua_reference_create, lua_reference_free, lua_reference_push,
    lua_remove, lua_set_field, lua_set_meta_table, lua_set_state, lua_set_table, lua_throw_error,
    lua_top, GarrysMod_Lua_CFunc, GarrysMod_Lua_ILuaBase,
};

use std::{
    any::Any,
    ffi::{CStr, CString},
    fmt::Debug,
    mem,
    pin::Pin,
};
use thiserror::Error;

pub struct LuaBase {
    lua_base: *mut GarrysMod_Lua_ILuaBase,
}

impl LuaBase {
    pub fn new(lua: *mut GarrysMod_Lua_ILuaBase) -> Self {
        LuaBase { lua_base: lua }
    }

    pub fn top(&self) -> i32 {
        unsafe { lua_top(self.lua_base) }
    }

    pub fn push(&self, stack_pos: i32) {
        unsafe { lua_push(self.lua_base, stack_pos) }
    }

    pub fn pop(&self, amount: i32) {
        unsafe { lua_pop(self.lua_base, amount) }
    }

    pub fn get_table(&self, stack_pos: i32) {
        unsafe { lua_get_table(self.lua_base, stack_pos) }
    }

    pub fn get_field(&self, stack_pos: i32, name: &str) {
        let c_name = CString::new(name).expect("Unable to create CString");
        unsafe { lua_get_field(self.lua_base, stack_pos, c_name.as_ptr()) }
    }

    pub fn set_field(&self, stack_pos: i32, name: &str) {
        let c_name = CString::new(name).expect("Unable to create CString");
        unsafe { lua_set_field(self.lua_base, stack_pos, c_name.as_ptr()) }
    }

    pub fn create_table(&self) {
        unsafe { lua_create_table(self.lua_base) }
    }

    pub fn set_table(&self, stack_pos: i32) {
        unsafe { lua_set_table(self.lua_base, stack_pos) }
    }

    pub fn set_meta_table(&self, stack_pos: i32) {
        unsafe { lua_set_meta_table(self.lua_base, stack_pos) }
    }

    pub fn get_meta_table(&self, i: i32) -> bool {
        unsafe { lua_get_meta_table(self.lua_base, i) }
    }

    pub fn call(&self, args: i32, results: i32) {
        unsafe { lua_call(self.lua_base, args, results) }
    }

    pub fn p_call(&self, args: i32, results: i32, error_func: i32) -> i32 {
        unsafe { lua_p_call(self.lua_base, args, results, error_func) }
    }

    pub fn equal(&self, a: i32, b: i32) -> i32 {
        unsafe { lua_equal(self.lua_base, a, b) }
    }

    pub fn raw_equal(&self, a: i32, b: i32) -> i32 {
        unsafe { lua_raw_equal(self.lua_base, a, b) }
    }

    pub fn insert(&self, stack_pos: i32) {
        unsafe { lua_insert(self.lua_base, stack_pos) }
    }

    pub fn remove(&self, stack_pos: i32) {
        unsafe { lua_remove(self.lua_base, stack_pos) }
    }

    pub fn next(&self, stack_pos: i32) -> i32 {
        unsafe { lua_next(self.lua_base, stack_pos) }
    }

    pub fn throw_error(&self, message: &str) {
        let c_message = CString::new(message).expect("Unable to create CString");
        unsafe { lua_throw_error(self.lua_base, c_message.as_ptr()) }
    }

    // TODO: use enum for lua_type
    pub fn check_type(&self, stack_pos: i32, lua_type: Type) {
        unsafe { lua_check_type(self.lua_base, stack_pos, lua_type as _) }
    }

    pub fn arg_error(&self, args: i32, message: &str) {
        let c_message = CString::new(message).expect("Unable to create CString");
        unsafe { lua_arg_error(self.lua_base, args, c_message.as_ptr()) }
    }

    pub fn raw_get(&self, stack_pos: i32) {
        unsafe { lua_raw_get(self.lua_base, stack_pos) }
    }

    pub fn raw_set(&self, stack_pos: i32) {
        unsafe { lua_raw_set(self.lua_base, stack_pos) }
    }

    pub fn get_string(&self, stack_pos: i32) -> Option<String> {
        unsafe {
            let c_val = lua_get_string(self.lua_base, stack_pos);

            let data: Vec<u8> = std::slice::from_raw_parts(c_val.str_ptr, c_val.str_len as _)
                .iter()
                .map(|&f| f as u8)
                .collect();

            if data.iter().any(|&f| f == 0) {
                None
            } else {
                let val = String::from_utf8_lossy(data.as_slice()).to_string();
                Some(val)
            }
        }
    }

    pub fn print(&self, value: &str) {
        self.push_special(SpecialType::GlobalTable);
        self.get_field(-1, "print");
        self.push_string(value);
        self.call(1, 0);
        self.pop(1);
    }

    pub fn push_number(&self, value: f64) {
        unsafe { lua_push_number(self.lua_base, value) }
    }

    /// # Safety
    ///
    /// This function will fail should the state pointer be invalid
    pub unsafe fn set_state(&self, state: *mut lua_State) {
        lua_set_state(self.lua_base, state)
    }

    pub fn push_special(&self, value_type: SpecialType) {
        unsafe { lua_push_special(self.lua_base, value_type as i32) }
    }

    // this will probably fail, rust might clean the string here before lua can use it
    pub fn push_string(&self, value: &str) {
        let c_value = CString::new(value).expect("Unable to create CString");
        unsafe { lua_push_string(self.lua_base, c_value.as_ptr(), value.len() as u32) }
    }

    pub fn push_c_function(&self, func: GarrysMod_Lua_CFunc) {
        unsafe { lua_push_c_function(self.lua_base, func) }
    }

    pub fn check_number(&self, stack_pos: i32) -> f64 {
        unsafe { lua_check_number(self.lua_base, stack_pos) }
    }

    pub fn check_string(&self, stack_pos: i32) -> String {
        unsafe {
            let c_val = lua_check_string(self.lua_base, stack_pos);
            CStr::from_ptr(c_val).to_str()
        }
        .expect("Failed to retrieve string.")
        .to_string()
    }

    pub fn get_bool(&self, stack_pos: i32) -> bool {
        unsafe { lua_get_bool(self.lua_base, stack_pos) }
    }

    pub fn get_c_func(&self, stack_pos: i32) -> GarrysMod_Lua_CFunc {
        unsafe { lua_get_c_func(self.lua_base, stack_pos) }
    }

    pub fn get_number(&self, stack_pos: i32) -> f64 {
        unsafe { lua_get_number(self.lua_base, stack_pos) }
    }

    pub fn get_type_name(&self, value_type: i32) -> String {
        unsafe {
            let c_val = lua_get_type_name(self.lua_base, value_type);
            CStr::from_ptr(c_val).to_str()
        }
        .expect("Failed to retrieve string.")
        .to_string()
    }

    pub fn get_type(&self, stack_pos: i32) -> i32 {
        unsafe { lua_get_type(self.lua_base, stack_pos) }
    }

    pub fn is_type(&self, stack_pos: i32, is_type: Type) -> bool {
        unsafe { lua_is_type(self.lua_base, stack_pos, is_type as _) }
    }

    pub fn obj_len(&self, stack_pos: i32) -> i32 {
        unsafe { lua_obj_len(self.lua_base, stack_pos) }
    }

    pub fn push_bool(&self, value: bool) {
        unsafe { lua_push_bool(self.lua_base, value) }
    }

    pub fn push_c_closure(&self, func: GarrysMod_Lua_CFunc, vars: i32) {
        unsafe { lua_push_c_closure(self.lua_base, func, vars) }
    }

    pub fn push_nil(&self) {
        unsafe { lua_push_nil(self.lua_base) }
    }

    pub fn reference_create(&self) -> i32 {
        unsafe { lua_reference_create(self.lua_base) }
    }

    pub fn reference_free(&self, raw_pointer: i32) {
        unsafe { lua_reference_free(self.lua_base, raw_pointer) }
    }

    pub fn reference_push(&self, raw_pointer: i32) {
        unsafe { lua_reference_push(self.lua_base, raw_pointer) }
    }

    pub fn push_usertype<T: Any>(&self, data: Box<T>) {
        unsafe {
            let raw_pointer: *mut u8 = mem::transmute(data);
            lua_push_usertype(self.lua_base, raw_pointer as _, Type::UserData as _)
        }
    }

    pub fn get_usertype<T: Any>(&self, stack_pos: i32) -> Option<Box<T>> {
        let ptr = unsafe { lua_get_usertype(self.lua_base, stack_pos, Type::UserData as _) };
        let result: Box<T> = unsafe { mem::transmute(ptr) };

        Some(result)
    }
}

pub enum SpecialType {
    GlobalTable = 0,
    Env = 1,
    Reg = 2,
}

pub enum Type {
    // Default Lua Types
    None = -1,
    Nil,
    Bool,
    LightUserData,
    Number,
    String,
    Table,
    Function,
    UserData,
    Thread,

    // GMod Types
    Entity,
    Vector,
    Angle,
    PhysObj,
    Save,
    Restore,
    DamageInfo,
    EffectData,
    MoveData,
    RecipientFilter,
    UserCmd,
    ScriptedVehicle,
    Material,
    Panel,
    Particle,
    ParticleEmitter,
    Texture,
    UserMsg,
    ConVar,
    IMesh,
    Matrix,
    Sound,
    PixelVisHandle,
    DLight,
    Video,
    File,
    Locomotion,
    Path,
    NavArea,
    SoundHandle,
    NavLadder,
    ParticleSystem,
    ProjectedTexture,
    PhysCollide,
    SurfaceInfo,
    TypeCount,
}

#[derive(Error, Debug)]
pub enum LuaWrapperError {
    #[error("Something went wrong")]
    Unknown,
}
