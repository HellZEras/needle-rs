use std::ptr::null_mut;
use winapi::um::{tlhelp32::TH32CS_SNAPPROCESS, winnt::{HANDLE, PROCESS_ALL_ACCESS}};
use crate::{errors::WinErros, memory::{create_tool_help_32_snapshot, get_module_base_name_a, open_process, process_32_next}, utils::{bytes2string, prepare_process_entry}};

#[derive(Debug)]
pub struct Process{
    pub handle : HANDLE,
    pub pid: u32,
    pub name: String,
}

impl Process {
    pub fn by_pid(process_id: u32) -> Result<Self,WinErros>{
        let process_handle = open_process(PROCESS_ALL_ACCESS, 0 as _, process_id)?;
        let mut buffer = [0u8;256];
        get_module_base_name_a(process_handle, null_mut(), buffer.as_mut_ptr() as _, buffer.len() as _)?;
        Ok(Process{
            handle:process_handle,
            pid: process_id,
            name: bytes2string(&mut buffer)
        })
        
    }

    pub fn first_by_name(name:&str) -> Result<Self,WinErros>{
        let all_processes = create_tool_help_32_snapshot(TH32CS_SNAPPROCESS, 0)?;
        let mut process_entry = prepare_process_entry();
        while process_32_next(all_processes, &mut process_entry)? {
            let process_name = bytes2string(&mut process_entry.szExeFile);
            if process_name.eq(name){
                return Process::by_pid(process_entry.th32ProcessID)
            }
        }
        Err(WinErros::ProcessNotFound)
    }
}