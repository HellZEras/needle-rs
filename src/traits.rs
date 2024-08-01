use crate::{dll::Dll, errors::MappedErrors};
use std::ffi::CStr;

pub trait AsCstr {
    fn as_cstr(&mut self) -> &CStr;
}

pub trait Injector {
    fn inject(&self, dll: Dll) -> Result<(), MappedErrors>;
}

pub trait ToU8Vec {
    fn to_u8_vec(self) -> Vec<u8>;
}

pub trait Is32bitsProcess {
    fn get_architecture(self) -> Result<bool, MappedErrors>;
}
pub trait MapMemory {
    fn find_sig(self, sig: &str) -> Vec<*mut u8>;
}
