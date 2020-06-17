use std::ffi::CString;
use std::ptr;
use winapi;
use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::{DWORD, HINSTANCE, LPVOID};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::winuser::MessageBoxA;

#[no_mangle]
pub unsafe extern "system" fn message_box(_: LPVOID) -> DWORD {
    let text = CString::new("Test").unwrap();
    MessageBoxA(ptr::null_mut(), text.as_ptr(), text.as_ptr(), 0);
    return 1;
}

#[no_mangle]
#[allow(non_snake_case, unused_variables, dead_code)]
pub extern "system" fn DllMain(h_inst: DWORD, reason: DWORD, _reserved: LPVOID) -> BOOL {
    unsafe {
        match reason {
            winapi::um::winnt::DLL_PROCESS_ATTACH => {
                winapi::um::processthreadsapi::CreateThread(
                    ptr::null_mut(),
                    0,
                    Some(message_box),
                    ptr::null_mut(),
                    0,
                    ptr::null_mut(),
                );
            }
            _ => (),
        };
    }

    return true as BOOL;
}
