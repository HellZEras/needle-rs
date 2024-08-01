pub use process::Process;
pub use dll::Dll;
mod win_funcs;
mod process;
mod utils;
mod errors;
mod traits;
mod dll;
mod injector;


#[cfg(test)]
mod tests {
    use dll::Dll;
    use traits::Injector;

    use super::*;

    #[test]
    fn it_works() {
        // let dll = Dll::new("D:\\Projects\\first_dll\\target\\release\\64dll.dll").unwrap();
        let dll = Dll::new("D:\\Projects\\first_dll\\target\\i686-pc-windows-msvc\\release\\32dll.dll").unwrap();
        let process = Process::first_by_name("notepad.exe").unwrap();
        
        process.inject(dll).unwrap()

    }
}