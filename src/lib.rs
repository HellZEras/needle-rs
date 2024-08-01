pub use dll::Dll;
pub use process::Process;
pub use traits::{Injector, MapMemory};
mod dll;
mod errors;
mod injector;
mod process;
mod traits;
mod utils;
mod win_funcs;

#[cfg(test)]
mod tests {
    use dll::Dll;
    use traits::Injector;

    use super::*;

    #[test]
    fn it_works() {
        // let dll = Dll::new("D:\\Projects\\first_dll\\target\\release\\64dll.dll").unwrap();
        let dll =
            Dll::new("D:\\Projects\\first_dll\\target\\i686-pc-windows-msvc\\release\\32dll.dll")
                .unwrap();
        let process = Process::first_by_name("notepad.exe").unwrap();

        process.inject(dll).unwrap()
    }
}
