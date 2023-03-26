// Just the part of https://github.com/Koka/gettext-rs/tree/master/gettext-sys that we want.
// The build script is not wanted and the main reason for not just using the crate as is.
// gtk::glib has some but not all gettext functions, so not all functions are needed.

use std::os::raw::{c_char, c_int};

extern "C" {
    pub fn bindtextdomain(domain: *const c_char, dir: *const c_char) -> *mut c_char;

    pub fn textdomain(domain: *const c_char) -> *mut c_char;

    pub fn bind_textdomain_codeset(domain: *const c_char, codeset: *const c_char) -> *mut c_char;

    pub fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
}
