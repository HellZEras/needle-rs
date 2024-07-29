use std::{ffi::CStr, io::Read, ptr::null_mut};
use winapi::{shared::minwindef::PUSHORT, um::{tlhelp32::PROCESSENTRY32, winnt::HANDLE, wow64apiset::IsWow64Process2}};

use crate::memory::get_last_error;

pub trait AsCstr {
    fn as_cstr(&mut self) -> &CStr;
}
impl<const N:usize> AsCstr for [i8;N] {
    fn as_cstr(&mut self) -> &CStr{
        unsafe { CStr::from_ptr(self.as_mut_ptr() as _)}    
    }
}
impl<const N:usize> AsCstr for [u8;N] {
    fn as_cstr(&mut self) -> &CStr{
        unsafe { CStr::from_ptr(self.as_mut_ptr() as _)}    
    }
}

pub fn bytes2string<T:AsCstr>(buffer: &mut T) -> String {
    let cstr = buffer.as_cstr();
    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

pub fn prepare_process_entry() -> PROCESSENTRY32 {
    let mut process_entry : PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    process_entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
    process_entry
}

pub fn is_syswow_64(
    h_process: HANDLE,
    p_process_machine: PUSHORT,
    p_native_machine: Option<PUSHORT>,
) -> Result<(), std::io::Error> {
    let func_result: i32 = unsafe { IsWow64Process2(h_process, p_process_machine, p_native_machine.unwrap_or(null_mut())) };
    if func_result == 0 {
        return Err(get_last_error("Failed to check process architecture"))
    }
    Ok(())
}

pub fn is_process_32bits(p_handle :HANDLE)-> Result<bool,std::io::Error>{
    let mut random : u16 = 1;
    is_syswow_64(p_handle as _, &mut random, None)?;
    Ok(random != 0)
}
pub fn is_dll_32bits(path: &str) -> Result<bool,std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Couldnt write file to buffer");
    let mut vec = Vec::new();
    buffer.iter().for_each(|&character| {
        if character.is_ascii_graphic() && character != b'\r' && character != b'\n' {
            vec.push(character as char)
        }
    });
    for (i,char) in vec.iter().enumerate(){
        if char.eq(&'P') && vec[i+1].eq(&'E') && vec[i+2].eq(&'d'){
            return Ok(false)
        }
    }
    Ok(true)    
}