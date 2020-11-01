use thiserror::Error;

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
    #[error("Global State error")]
    GlobalStateAccessError,
}
