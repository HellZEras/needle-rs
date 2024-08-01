use std::{ffi::CStr, io::Read};
use winapi::um::tlhelp32::PROCESSENTRY32;
use crate::{errors::MethodErrors, traits::{AsCstr, FirstChar, Increment, RemoveSpaces, ToU8Vec}};


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

impl FirstChar for &str {
    fn first(self) -> Result<char,MethodErrors>{
        if let Some(var) = self.chars().next(){
            return Ok(var)
        }
        Err(MethodErrors::FirstElementError)
    }
}

impl Increment for usize {
    fn inc(&mut self) -> usize{
        *self += 1;
        *self
    }
}

impl RemoveSpaces for &str {
    fn pop_spaces(self) -> String {
        self.replace(' ', "")
    }
}

impl ToU8Vec for &str {
    fn to_u8_vec(self) -> Vec<u8> {
        self.split_whitespace()
            .map(|c| {
                u8::from_str_radix(c, 16).unwrap_or(0)
            })
            .collect()
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


pub fn is32_bits_dll(file_contents:Vec<char>) -> Result<bool, std::io::Error> {
    for (i,char) in file_contents.iter().enumerate(){
        if char.eq(&'P') && file_contents[i+1].eq(&'E') && file_contents[i+2].eq(&'d'){
            return Ok(false)
        }
    }
    Ok(true)    
}

pub fn init_contents(path :&str)-> Result<Vec<char>,std::io::Error>{
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Couldnt write file to buffer");
    let mut buffer_of_contents = Vec::new();
    buffer.iter().for_each(|&character| {
        if character.is_ascii_graphic() && character != b'\r' && character != b'\n' {
            buffer_of_contents.push(character as char)
        }
    });
    Ok(buffer_of_contents)
}
