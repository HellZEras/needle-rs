use std::{mem::size_of, ptr::null_mut};
use winapi::um::{tlhelp32::TH32CS_SNAPPROCESS, winnt::{HANDLE, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_READONLY, PAGE_READWRITE, PROCESS_ALL_ACCESS}};
use crate::{errors::MappedErrors, traits::{Is32bitsProcess, MapMemory, ToU8Vec}, utils::{bytes2string, prepare_process_entry}, win_funcs::{create_tool_help_32_snapshot, get_module_base_name_a, is_syswow_64, open_process, process_32_next, read_process_memory, virtual_query_ex}};

#[derive(Debug)]
pub struct Process{
    pub handle : HANDLE,
    pub pid: u32,
    pub name: String,
    pub architecture : i32
}
impl Is32bitsProcess for HANDLE{
    fn get_architecture(self)-> Result<bool,MappedErrors>{
        let mut random : u16 = 1;
        is_syswow_64(self, &mut random, None)?;
        Ok(random != 0)
    }
}

impl Process {
    pub fn by_pid(process_id: u32) -> Result<Self,MappedErrors> {
        let process_handle = open_process(PROCESS_ALL_ACCESS, 0 as _, process_id)?;
        let mut buffer = [0u8;256];
        get_module_base_name_a(process_handle, null_mut(), buffer.as_mut_ptr() as _, buffer.len() as _)?;
        Ok(Process{
            handle:process_handle,
            pid: process_id,
            name: bytes2string(&mut buffer),
            architecture: if process_handle.get_architecture()?{32}else{64}
        })
    }

    pub fn first_by_name(name:&str) -> Result<Self,MappedErrors>{
        let all_processes = create_tool_help_32_snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut process_entry = prepare_process_entry();
        while process_32_next(all_processes, &mut process_entry)? {
            let process_name = bytes2string(&mut process_entry.szExeFile);
            if process_name.eq(name){
                return Process::by_pid(process_entry.th32ProcessID)
            }
        }
        Err(MappedErrors::ProcessNotFound)
    }

}

impl MapMemory for Process{
    fn find_sig(self,sig:&str) -> Vec<*mut u8>{
        let mut addy: usize = 0;
        let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        let mut bytes_read = 0;
        let mut matches : Vec<*mut u8> = Vec::new();
        let sig_vec = sig.to_u8_vec();
        while virtual_query_ex(self.handle, addy as *const _, &mut mbi, size_of::<MEMORY_BASIC_INFORMATION>()) {
            if mbi.State == MEM_COMMIT && (mbi.Protect != PAGE_READWRITE || mbi.Protect != PAGE_READONLY) {
                let mut buffer = vec![0u8; mbi.RegionSize];
                if read_process_memory(self.handle, addy as *const _, buffer.as_mut_ptr() as *mut _, mbi.RegionSize, Some(&mut bytes_read)) {
                    if buffer.is_empty() {
                        continue;
                    }
                    for i in 0..bytes_read-sig_vec.len(){
                        let mut found = true;
                        for j in 0..sig_vec.len(){
                            if sig_vec[j] != 0 && buffer[i+j] != sig_vec[j]{
                                found = false;
                                break;
                            }
                        }
                        if found {
                            matches.push((mbi.BaseAddress as usize + i) as *mut u8)
                        }
                    }
                }
            }
            addy += mbi.RegionSize;
        }
        matches
        }
}