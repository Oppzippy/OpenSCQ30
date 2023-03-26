// Minimal version of https://github.com/Koka/gettext-rs/tree/master/gettext-rs only containing what we need
// gtk::glib has some but not all gettext functions

use std::{
    ffi::{CStr, CString},
    io,
};

use crate::gettext_sys;

pub fn bindtextdomain(domain: &str, dir: &str) -> Result<(), io::Error> {
    let domain = CString::new(domain).unwrap();
    let dir = CString::new(dir).unwrap();
    let result = unsafe { gettext_sys::bindtextdomain(domain.as_ptr(), dir.as_ptr()) };
    if result.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn textdomain(domain: &str) -> Result<(), io::Error> {
    let domain = CString::new(domain).unwrap();
    let result = unsafe { gettext_sys::textdomain(domain.as_ptr()) };
    if result.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub fn bind_textdomain_codeset(domain: &str, codeset: &str) -> Result<(), io::Error> {
    let domain = CString::new(domain).unwrap();
    let codeset = CString::new(codeset).unwrap();
    let result = unsafe { gettext_sys::bind_textdomain_codeset(domain.as_ptr(), codeset.as_ptr()) };
    if result.is_null() {
        let err = io::Error::last_os_error();
        if let Some(0) = err.raw_os_error() {
            // Not sure what 0 means where but ignoring it is what gettext-rs does
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    } else {
        Ok(())
    }
}

pub fn setlocale(category: i32, locale: &str) -> Option<String> {
    let locale = CString::new(locale).unwrap();
    let result = unsafe { gettext_sys::setlocale(category, locale.as_ptr()) };
    if result.is_null() {
        None
    } else {
        let selected_locale_c_str = unsafe { CStr::from_ptr(result) };
        Some(selected_locale_c_str.to_str().unwrap().to_owned())
    }
}
