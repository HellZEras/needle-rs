use std::ffi::CStr;
use crate::{dll::Dll, errors::{MethodErrors, WinErrors}};

pub trait AsCstr {
    fn as_cstr(&mut self) -> &CStr;
}

pub trait Injector {
    fn inject(&self, dll: Dll) -> Result<(), WinErrors>;
}

pub trait FirstChar {
    fn first(self) -> Result<char,MethodErrors>;
}

pub trait Increment {
    fn inc(&mut self) -> usize;
}

pub trait RemoveSpaces {
    fn pop_spaces(self) -> String;
}

pub trait ToU8Vec {
    fn to_u8_vec(self) -> Vec<u8>;
}

pub trait Is32bitsProcess{
    fn get_architecture(self)-> Result<bool,WinErrors>;
}
pub trait MapMemory{
    fn find_sig(self,sig:&str) -> Vec<*mut u8>;
}