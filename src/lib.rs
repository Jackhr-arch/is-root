#[cfg(windows)]
pub fn is_root() -> bool {
    use std::mem;
    use winapi::{
        ctypes::c_void,
        shared::minwindef::{DWORD, TRUE},
        um::{
            handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
            processthreadsapi::{GetCurrentProcess, OpenProcessToken},
            securitybaseapi::GetTokenInformation,
            winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY},
        },
    };

    let mut token = INVALID_HANDLE_VALUE;
    let mut elevated = false;
    unsafe {
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == TRUE {
            let mut elevation: TOKEN_ELEVATION = mem::zeroed();
            let mut size = mem::size_of::<TOKEN_ELEVATION>() as DWORD;
            if GetTokenInformation(
                token,
                TokenElevation,
                &mut elevation as *mut TOKEN_ELEVATION as *mut c_void,
                size,
                &mut size,
            ) == TRUE
            {
                elevated = elevation.TokenIsElevated != 0;
            }
        }

        if token != INVALID_HANDLE_VALUE {
            CloseHandle(token);
        }
    }

    elevated
}

#[cfg(unix)]
pub fn is_root() -> bool {
    users::get_current_uid() == 0
}
