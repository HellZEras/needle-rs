use winapi::{shared::{basetsd::SIZE_T, minwindef::{BOOL, DWORD, FARPROC, HMODULE, LPCVOID, LPDWORD, LPVOID, PUSHORT}, ntdef::LPSTR}, um::{libloaderapi::{GetModuleHandleA, GetProcAddress}, memoryapi::{ReadProcessMemory, VirtualAllocEx, VirtualQueryEx, WriteProcessMemory}, minwinbase::{LPSECURITY_ATTRIBUTES, LPTHREAD_START_ROUTINE}, processthreadsapi::{CreateRemoteThread, OpenProcess}, psapi::GetModuleBaseNameA, synchapi::WaitForSingleObject, tlhelp32::{CreateToolhelp32Snapshot, Process32Next, LPPROCESSENTRY32}, winnt::{HANDLE, LPCSTR, PMEMORY_BASIC_INFORMATION}, wow64apiset::IsWow64Process2}};
use std::ptr::null_mut;
use crate::errors::WinErrors::{self, *};

const WAIT_TIMEOUT :u32 = 0x00000102;
const WAIT_FAILED :u32 = 0xFFFFFFFF;
const WAIT_OBJECT_0 :u32 = 0x00000000;
const WAIT_ABANDONED :u32 = 0x00000080;


pub fn allocate_memory(hprocess: HANDLE,
    lpaddress: LPVOID,
    dwsize: SIZE_T,
    flallocationtype: DWORD,
    flprotect: DWORD,
) -> Result<LPVOID,WinErrors>{
    let buffer = unsafe { VirtualAllocEx(hprocess, lpaddress, dwsize, flallocationtype, flprotect)};
    match buffer.is_null(){
        true => Err(MemoryAllocationFailure),
        false =>  Ok(buffer)
    }

}

pub fn write_process_memory(hprocess: HANDLE,
    lp_baseaddress: LPVOID,
    lp_buffer: LPCVOID,
    nsize: SIZE_T,
    lp_number_of_bytes_written: Option< *mut SIZE_T >,
) -> Result<(),WinErrors> {
    let func_return = unsafe { WriteProcessMemory(hprocess, lp_baseaddress, lp_buffer, nsize, lp_number_of_bytes_written.unwrap_or(null_mut()) ) };
    if func_return == 0{
        return Err(MemoryWritingFailure)
    }
        Ok(())
}

pub fn get_module_handle_a(
    lp_module_name: LPCSTR,
) -> Result<HMODULE,WinErrors> {
    let handle = unsafe { GetModuleHandleA(lp_module_name) };
    match handle.is_null() {
        true => Err(ModuleHandleFailure),
        false => Ok(handle)
    }
}

pub fn get_proc_address(
    h_module: HMODULE,
    lp_proc_name: LPCSTR,
) -> Result<FARPROC,WinErrors>{
    let func = unsafe { GetProcAddress(h_module, lp_proc_name) };
    match func.is_null(){
        true => Err(ProcAddressFailure),
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
) -> Result<HANDLE,WinErrors> 
    {
        let thread = unsafe { CreateRemoteThread(h_process,lp_thread_attributes,dw_stack_size,lp_start_address,lp_parameter,dw_creation_flags,lp_thread_id) };
        match thread.is_null() {
            true => Err(ThreadCreationFailure),
            false => Ok(thread)
    }
}

pub fn wait_for_single_object(
    h_handle: HANDLE,
    dw_milliseconds: DWORD,
) -> Result<(),WinErrors>{
    let wait_result = unsafe { WaitForSingleObject(h_handle, dw_milliseconds) };
    match wait_result {
        WAIT_ABANDONED => Err(WaitForSingleObjectAbandoned),
        WAIT_FAILED => Err(WaitForSingleObjectFailure),
        WAIT_OBJECT_0 => Ok(()),
        WAIT_TIMEOUT => Err(WaitForSingleObjectTimeOut),
        _ => Err(WaitForSingleObjectUnknown)
    }
}
pub fn open_process(
    dw_desired_access: DWORD,
    b_inherit_handle: BOOL,
    dw_process_id: DWORD,
) -> Result<HANDLE,WinErrors>{
    let process_handle = unsafe { OpenProcess(dw_desired_access, b_inherit_handle, dw_process_id)};
    match process_handle.is_null(){
        true => Err(ProcessOpeningFailure),
        false => Ok(process_handle)
    }
}

pub fn get_module_base_name_a(
    h_process: HANDLE,
    h_module: HMODULE,
    lp_base_name: LPSTR,
    n_size: DWORD,
) -> Result<DWORD,WinErrors>{
    let func = unsafe { GetModuleBaseNameA(h_process, h_module, lp_base_name, n_size)};
    if func == 0{
        return Err(ModuleBaseNameFailure)
    }
    Ok(func)
}

pub fn create_tool_help_32_snapshot(
    dw_flags: DWORD,
    th32_process_id: DWORD,
) -> Result<HANDLE,WinErrors> {
    let all_processes = unsafe { CreateToolhelp32Snapshot(dw_flags, th32_process_id) };
    if all_processes.is_null() {
        return Err(SnapshotCreationFailure)
    }
    Ok(all_processes)
}

pub fn process_32_next(
    h_snapshot: HANDLE,
    lppe: LPPROCESSENTRY32,
) -> Result<bool,WinErrors> {
    let next_process_bool = unsafe { Process32Next(h_snapshot, lppe) };
    match next_process_bool {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(NextProcessFailure)
    }
}

pub fn is_syswow_64(
    h_process: HANDLE,
    p_process_machine: PUSHORT,
    p_native_machine: Option<PUSHORT>,
) -> Result<(), WinErrors> {
    let func_result: i32 = unsafe { IsWow64Process2(h_process, p_process_machine, p_native_machine.unwrap_or(null_mut())) };
    if func_result == 0 {
        return Err(WinErrors::ProcessCheckFailure)
    }
    Ok(())
}

pub fn read_process_memory(
    h_process: HANDLE,
    lp_base_address: LPCVOID,
    lp_buffer: LPVOID,
    n_size: SIZE_T,
    lp_number_of_bytes_read: Option<*mut SIZE_T>,
) -> bool {
    unsafe { ReadProcessMemory(h_process, lp_base_address, lp_buffer, n_size, lp_number_of_bytes_read.unwrap_or(null_mut())) != 0 } 
}

pub fn virtual_query_ex(
    h_process: HANDLE,
    lp_address: LPCVOID,
    lp_buffer: PMEMORY_BASIC_INFORMATION,
    dw_length: SIZE_T,
) -> bool{
    let val = unsafe { VirtualQueryEx(h_process, lp_address, lp_buffer, dw_length)};
    val != 0
}
