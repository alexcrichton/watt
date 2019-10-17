use std::slice;
use std::str;

mod engine;
mod ffi;
mod func;
mod functype;
mod instance;
mod module;
mod store;
mod trap;
mod val;
mod valtype;
pub use crate::engine::*;
pub use crate::func::*;
pub use crate::functype::*;
pub use crate::instance::*;
pub use crate::module::*;
pub use crate::store::*;
pub use crate::trap::*;
pub use crate::val::*;
pub use crate::valtype::*;

unsafe fn name_to_str<'a>(name: *const ffi::wasm_name_t) -> &'a str {
    str::from_utf8_unchecked(slice::from_raw_parts((*name).data, (*name).size))
}
