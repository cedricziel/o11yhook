use std::{ffi::{CStr, CString}, sync::Mutex};
use libc::c_char;

use lazy_static::lazy_static;

extern crate libc;

#[macro_use]
extern crate redhook;

static PHP_INI_PATH: &str = "/etc/php/7.4/cli.ini";
lazy_static! {
    static ref PHP_INI_PATH_CSTR: Mutex<CString> = Mutex::new(CString::new(PHP_INI_PATH).unwrap());
}

hook! {
    unsafe fn getenv(c_name: *const c_char) -> *const c_char => o11yhook_getenv {
        let retval = real!(getenv)(c_name);

        if let Ok(name) = CStr::from_ptr(c_name).to_str() {
            if name == "PHPRC" {
                let php_ini_path_cstr = PHP_INI_PATH_CSTR.lock().unwrap();
                return php_ini_path_cstr.as_ptr();
            }
        }

        retval
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::CString;
    use libc::getenv;

    #[test]
    fn test_getenv_phprc() {
        unsafe {
            let name = CString::new("PHPRC").unwrap();
            let phprc = getenv(name.as_ptr());
            let phprc_str = CStr::from_ptr(phprc).to_str().unwrap();

            assert_eq!(phprc_str, "/etc/php/7.4/cli.ini");
        }
    }

    #[test]
    fn test_getenv_path() {
        unsafe {
            let name = CString::new("PATH").unwrap();
            let phprc = getenv(name.as_ptr());
            let phprc = std::str::from_utf8(CStr::from_ptr(phprc).to_bytes()).unwrap();

            assert_ne!(phprc, "");
        }
    }
}