#![deny(warnings, clippy::all)]
#![warn(missing_docs)]
//! A simple library to detect whether you are root/admin or not
//! ## Usage
//! ```no_run
//! use is_root::is_root;
//!
//! if is_root() {
//!     println!("Doing something dangerous")
//! } else {
//!     eprintln!("Run me as root")
//! }
//! ```

/// Returns `true` if user is root; `false` otherwise
/// ```no_run
/// use is_root::is_root;
///
/// if is_root() {
///     println!("Doing something dangerous")
/// } else {
///     eprintln!("Run me as root")
/// }
/// ```
#[must_use]
pub fn is_root() -> bool {
    is_root_internal()
}

#[cfg(windows)]
#[must_use]
fn is_root_internal() -> bool {
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
#[must_use]
fn is_root_internal() -> bool {
    users::get_current_uid() == 0
}
