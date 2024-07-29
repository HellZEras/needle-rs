use std::ptr::null_mut;
use winapi::um::{handleapi::CloseHandle, winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE}};
use crate::{memory::{allocate_memory, create_remote_thread, get_module_handle_a, get_proc_address, wait_for_single_object, write_process_memory}, process::Process};

mod memory;
mod process;
mod utils;

const INFINITE :u32 = 0xFFFFFFFF;


pub trait Injector {
    fn inject(&self, dll: &str) -> Result<(), std::io::Error>;
}

impl Injector for Process {
    fn inject(&self, dll_path: &str) -> Result<(), std::io::Error> {
        let dll_c_path = std::ffi::CString::new(dll_path).unwrap();
        let dll_size = dll_c_path.as_bytes_with_nul().len()+1;
        
        // Allocate memory
        let buffer = allocate_memory(self.handle, null_mut(), dll_size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE)?;
        
        // Write DLL path to the allocated memory
        write_process_memory(self.handle, buffer, dll_c_path.as_ptr() as _, dll_size, None)?;
        
        // Get handle to kernel32.dll
        let kernel_string = std::ffi::CString::new("kernel32.dll").unwrap();
        let kernel32_handle = get_module_handle_a(kernel_string.as_ptr() as _)?;
        
        // Get the address of LoadLibraryA
        let loadlib_stirng = std::ffi::CString::new("LoadLibraryA").unwrap();
        let loadlib_func = get_proc_address(kernel32_handle, loadlib_stirng.as_ptr() as _)?;
        
        // Create a remote thread that calls LoadLibraryA with our DLL path
        let thread = create_remote_thread(self.handle, null_mut(), 0, unsafe { std::mem::transmute(loadlib_func) }, buffer, 0, null_mut())?;
        
        // Wait for the thread to complete
        wait_for_single_object(thread, INFINITE)?;
        
        // Close the handle to the thread
        unsafe { CloseHandle(thread) };
        
        Ok(())
    }
}

