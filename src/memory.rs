use winapi::{shared::{basetsd::SIZE_T, minwindef::{BOOL, DWORD, FARPROC, HMODULE, LPCVOID, LPDWORD, LPVOID, PUSHORT}, ntdef::LPSTR}, um::{errhandlingapi::GetLastError, libloaderapi::{GetModuleHandleA, GetProcAddress}, memoryapi::{VirtualAllocEx, WriteProcessMemory}, minwinbase::{LPSECURITY_ATTRIBUTES, LPTHREAD_START_ROUTINE}, processthreadsapi::{CreateRemoteThread, OpenProcess}, psapi::GetModuleBaseNameA, synchapi::WaitForSingleObject, tlhelp32::{CreateToolhelp32Snapshot, Process32Next, LPPROCESSENTRY32}, winnt::{HANDLE, LPCSTR}, wow64apiset::IsWow64Process2}};
use std::{io::{Error, Read}, ptr::null_mut};

const WAIT_TIMEOUT :u32 = 0x00000102;
const WAIT_FAILED :u32 = 0xFFFFFFFF;
const WAIT_OBJECT_0 :u32 = 0x00000000;
const WAIT_ABANDONED :u32 = 0x00000080;

pub fn get_last_error(error_txt: &str ) -> std::io::Error {
    std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("{}: {}",error_txt, unsafe { GetLastError() })
    )
}

pub fn allocate_memory(hprocess: HANDLE,
    lpaddress: LPVOID,
    dwsize: SIZE_T,
    flallocationtype: DWORD,
    flprotect: DWORD,
) -> Result<LPVOID,Error>{
    let buffer = unsafe { VirtualAllocEx(hprocess, lpaddress, dwsize, flallocationtype, flprotect)};
    match buffer.is_null(){
        true => Err(get_last_error("Memory allocation failed")),
        false =>  Ok(buffer)
    }

}

pub fn write_process_memory(hprocess: HANDLE,
    lp_baseaddress: LPVOID,
    lp_buffer: LPCVOID,
    nsize: SIZE_T,
    lp_number_of_bytes_written: Option< *mut SIZE_T >,
) -> Result<(),Error> {
    match lp_number_of_bytes_written{
        Some(bytes_written) => {
            let func_return = unsafe { WriteProcessMemory(hprocess, lp_baseaddress, lp_buffer, nsize, bytes_written ) };
            if func_return == 0{
                Err(get_last_error("Memory Writing failed"))
            }
            else{
                Ok(())
            }
        },
        None => {
            let func_return = unsafe { WriteProcessMemory(hprocess, lp_baseaddress, lp_buffer, nsize, null_mut() ) };
            if func_return == 0{
                Err(get_last_error("Memory Writing failed"))
            }
            else{
                Ok(())
            }
        }
    }
}

pub fn get_module_handle_a(
    lp_module_name: LPCSTR,
) -> Result<HMODULE,Error> {
    let handle = unsafe { GetModuleHandleA(lp_module_name) };
    match handle.is_null() {
        true => Err(get_last_error("Getting module handle failed")),
        false => Ok(handle)
    }
}

pub fn get_proc_address(
    h_module: HMODULE,
    lp_proc_name: LPCSTR,
) -> Result<FARPROC,Error>{
    let func = unsafe { GetProcAddress(h_module, lp_proc_name) };
    match func.is_null(){
        true => Err(get_last_error("Getting function failed")),
        false => Ok(func)
    }
}

pub fn create_remote_thread(
    h_process: HANDLE,
    lp_thread_attributes: LPSECURITY_ATTRIBUTES,
    dw_stack_size: SIZE_T,
    lp_start_address: LPTHREAD_START_ROUTINE,
    lp_parameter: LPVOID,
    dw_creation_flags: DWORD,
    lp_thread_id: LPDWORD,
) -> Result<HANDLE,Error> 
    {
        let thread = unsafe { CreateRemoteThread(h_process,lp_thread_attributes,dw_stack_size,lp_start_address,lp_parameter,dw_creation_flags,lp_thread_id) };
        match thread.is_null() {
            true => Err(get_last_error("Thread creation failed")),
            false => Ok(thread)
    }
}

pub fn wait_for_single_object(
    h_handle: HANDLE,
    dw_milliseconds: DWORD,
) -> Result<(),Error>{
    let wait_result = unsafe { WaitForSingleObject(h_handle, dw_milliseconds) };
    match wait_result {
        WAIT_ABANDONED => Err(get_last_error("Wait for single object abandoned")),
        WAIT_FAILED => Err(get_last_error("Wait for single object failed")),
        WAIT_OBJECT_0 => Ok(()),
        WAIT_TIMEOUT => Err(get_last_error("Wait for single object timed out")),
        _ => Err(get_last_error("Unknown error"))
    }
}
pub fn open_process(
    dw_desired_access: DWORD,
    b_inherit_handle: BOOL,
    dw_process_id: DWORD,
) -> Result<HANDLE,Error>{
    let process_handle = unsafe { OpenProcess(dw_desired_access, b_inherit_handle, dw_process_id)};
    match process_handle.is_null(){
        true => Err(get_last_error("Failed to open process")),
        false => Ok(process_handle)
    }
}

pub fn get_module_base_name_a(
    h_process: HANDLE,
    h_module: HMODULE,
    lp_base_name: LPSTR,
    n_size: DWORD,
) -> Result<DWORD,Error>{
    let func = unsafe { GetModuleBaseNameA(h_process, h_module, lp_base_name, n_size)};
    match func{
        0 => Err(get_last_error("Failed to open process")),
        _ => Ok(func)
    }
}

pub fn create_tool_help_32_snapshot(
    dw_flags: DWORD,
    th32_process_id: DWORD,
) -> Result<HANDLE,Error> {
    let all_processes = unsafe { CreateToolhelp32Snapshot(dw_flags, th32_process_id) };
    match all_processes.is_null(){
        true => Err(get_last_error("Failed get th32snapshot")),
        false => Ok(all_processes)
    }
}

pub fn process_32_next(
    h_snapshot: HANDLE,
    lppe: LPPROCESSENTRY32,
) -> Result<bool,Error> {
    let next_process_bool = unsafe { Process32Next(h_snapshot, lppe) };
    match next_process_bool {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(get_last_error("Error Getting next process"))
    }
}

