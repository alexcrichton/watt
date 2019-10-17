use crate::ffi;
use std::mem;
use std::fmt;

#[repr(transparent)]
pub struct Trap {
    pub(crate) raw: *mut ffi::wasm_trap_t,
}

impl Trap {
    pub fn message(&self) -> Message {
        unsafe {
            let mut raw = mem::zeroed();
            ffi:: wasm_trap_message(self.raw, &mut raw);
            Message { raw }
        }
    }
}

impl fmt::Display for Trap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message().as_str().fmt(f)
    }
}

impl fmt::Debug for Trap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Drop for Trap {
    fn drop(&mut self) {
        unsafe {
            ffi::wasm_trap_delete(self.raw);
        }
    }
}

#[repr(transparent)]
pub struct Message {
    pub(crate) raw: ffi::wasm_message_t,
}

impl Message {
    pub fn as_str(&self) -> &str {
        unsafe {
            crate::name_to_str(&self.raw)
        }
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            ffi::wasm_byte_vec_delete(&mut self.raw);
        }
    }
}
