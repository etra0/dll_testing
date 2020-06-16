use memory_rs::process::process_wrapper::Process;
use std::env;
use std::ffi::CString;
use std::mem;
use std::process;
use std::ptr;
use winapi::shared::basetsd::DWORD_PTR;
use winapi::shared::minwindef::LPVOID;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::{FreeLibrary, GetModuleHandleA, GetProcAddress};
use winapi::um::memoryapi::VirtualAllocEx;
use winapi::um::minwinbase::LPTHREAD_START_ROUTINE;
use winapi::um::processthreadsapi::CreateRemoteThread;
use winapi::um::winbase::FormatMessageA;
use winapi::um::winnt::{HANDLE, MEM_COMMIT, PAGE_READWRITE};

pub fn get_base_offset(process: &Process) {
    let dll_dir = CString::new(
        "C:\\Users\\Sebastian\\Documents\\work\\rust\\test_dll\\target\\debug\\testlib.dll",
    )
    .unwrap();
    let dll_dir_s = dll_dir.as_bytes_with_nul().len();
    unsafe {
        let mut buffer: Vec<i8> = vec![0; 64];

        let s_module_handle = CString::new("Kernel32").unwrap();
        let module_handle = GetModuleHandleA(s_module_handle.as_ptr());
        println!("{:x}", module_handle as u64);

        let name = CString::new("LoadLibraryA").unwrap();
        let result = GetProcAddress(module_handle, name.as_ptr());
        let casted_function: extern "system" fn(LPVOID) -> u32 = mem::transmute(result);
        let lpthread: LPTHREAD_START_ROUTINE = Some(casted_function);

        println!("{:x}", result as u64);

        let addr = VirtualAllocEx(
            process.h_process,
            ptr::null_mut(),
            dll_dir_s,
            MEM_COMMIT,
            PAGE_READWRITE,
        ) as DWORD_PTR;

        let _dll_dir = dll_dir.into_bytes_with_nul();

        process.write_aob(addr, &_dll_dir, true);

        println!("DLL address {:x}", addr);

        let a = CreateRemoteThread(
            process.h_process,
            ptr::null_mut(),
            0,
            lpthread,
            addr as LPVOID,
            0,
            ptr::null_mut(),
        );
        println!("handle {:x?}", a);
        let last_err = GetLastError();

        print!("0x{:x} ", last_err);

        FormatMessageA(
            0x1000,
            std::ptr::null(),
            last_err,
            0,
            buffer.as_mut_ptr(),
            64,
            std::ptr::null_mut(),
        );

        FreeLibrary(module_handle);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Not enough arguments.");
        process::exit(0);
    }

    let [_, p_name, dll_dir] = args[..3];
    let process = Process::new(&p_name).unwrap();
    get_base_offset(&process)
}
