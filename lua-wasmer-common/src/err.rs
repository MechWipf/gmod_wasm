use thiserror::Error;

#[derive(Error, Debug)]
pub enum LuaWasmerError {
    #[error("Invalid string")]
    InvalidString,
    #[error("Unable to convert wat to wasm")]
    Wat2Wasm,
    #[error("Compile error {message:?}")]
    CompileError { message: String },
    #[error("Unable to create instance {message:?}")]
    InstanceError { message: String },
    #[error("Did you specify a main method? {message:?}")]
    NoMainMethodFound { message: String },
    #[error("World is broken {message:?}")]
    RuntimeError { message: String },
}
