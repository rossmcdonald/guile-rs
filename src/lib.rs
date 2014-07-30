#![crate_name = "guile"]
#![crate_type = "lib"]
#![feature(globs)]

extern crate libc;

use ffi::*;

mod ffi;

pub fn scm_init_guile() {
    unsafe {
        ffi::scm_init_guile();
    }
}
