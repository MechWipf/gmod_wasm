# Gmod Web Assembly Module
Web Assembly modules usable in Garry's Mod glua
__This module is currently very WiP and not in a usable state!__

## How to compile

Install rust and cargo.

Fetch `Interface.h, LuaBase.h, SourceCompat.h, Types.h, and UserData.h` from somewhere and add them to  
`crates/gmod-sys/include/GarrysMod/Lua`

I can't provide them here because Garry did not add a license for those files.

Just run `cargo build`.  
Add `--release` for a release build with better optimization.  
Copy `target/(release|debug)/gmsv_wasm_win64.dll` into `garrysmod/lua/bin`

Tested on windows x86_64, rustc 1.47.0 on the Gmod x64 branch