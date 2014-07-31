#![crate_name = "guile"]
#![crate_type = "lib"]
#![feature(globs)]

extern crate libc;

use ffi::*;

use std::ptr;

use std::{
    i64,
    i32,
    i16,
    i8,
    u64,
    u32,
    u16,
    u8,
    f64
};

mod ffi;


pub struct Scm {
    n: ffi::SCM
}

impl ToPrimitive for Scm {
    fn to_i64(&self) -> Option<i64> {
        unsafe {
            Some(ffi::scm_to_int64(self.n))
        }
    }
    fn to_u64(&self) -> Option<u64> {
        unsafe {
            Some(ffi::scm_to_uint64(self.n))
        }
    }

    fn to_int(&self) -> Option<int> {
        unsafe {
            Some(ffi::scm_to_int32(self.n) as int)
        }
    }
    fn to_i8(&self) -> Option<i8> {
        unsafe {
            if ffi::scm_from_int8(1) == ffi::scm_leq_p(
                self.n, ffi::scm_from_int8(i8::MAX)) {
                Some(ffi::scm_to_int8(self.n))
            } else {
                None
            }
        }
    }
    fn to_i16(&self) -> Option<i16> {
        unsafe {
            Some(ffi::scm_to_int16(self.n))
        }
    }
    fn to_i32(&self) -> Option<i32> {
        unsafe {
            Some(ffi::scm_to_int32(self.n))
        }
    }
    fn to_uint(&self) -> Option<uint> {
        unsafe {
            Some(ffi::scm_to_uint32(self.n) as uint)
        }
    }
    fn to_u8(&self) -> Option<u8> {
        unsafe {
            Some(ffi::scm_to_uint8(self.n))
        }
    }
    fn to_u16(&self) -> Option<u16> {
        unsafe {
            Some(ffi::scm_to_uint16(self.n))
        }
    }
    fn to_u32(&self) -> Option<u32> {
        unsafe {
            Some(ffi::scm_to_uint32(self.n))
        }
    }
    fn to_f32(&self) -> Option<f32> {
        unsafe {
            Some(ffi::scm_to_double(self.n) as f32)
        }
    }
    fn to_f64(&self) -> Option<f64> {
        unsafe {
            Some(ffi::scm_to_double(self.n) as f64)
        }
    }
}
pub trait ToScm {
    fn to_scm(&self) -> Scm;
}

impl ToScm for i8 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_int8(*self))
        }
    }
}

impl ToScm for u8 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_uint8(*self))
        }
    }
}


impl ToScm for i16 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_int16(*self))
        }
    }
}

impl ToScm for u16 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_uint16(*self))
        }
    }
}

impl ToScm for i32 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_int32(*self))
        }
    }
}

impl ToScm for u32 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_uint32(*self))
        }
    }
}


impl ToScm for i64 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_int64(*self))
        }
    }
}

impl ToScm for u64 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_uint64(*self))
        }
    }
}

impl ToScm for f64 {
    fn to_scm(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_double(*self))
        }
    }
}

impl Scm {
    fn new(n: ffi::SCM) -> Scm{
        Scm {
            n: n
        }
    }

    pub fn to_int32(&self) -> i32{
        unsafe {
            scm_to_int32(self.n)
        }
    }

    pub fn call<T: ToScm>(&self, args: Vec<T>) -> Scm {
        let mut s: Vec<SCM> = args.iter().map(|x| x.to_scm().n).collect();
        unsafe {
            let len = s.len();
            Scm::new(ffi::scm_call_n(
                self.n, s.as_mut_slice(), len as u64))
        }
    }
    pub fn from_int32(num: i32) -> Scm {
        unsafe {
            Scm::new(ffi::scm_from_int32(num))
        }
    }
    pub fn c_lookup(s: &str) -> Scm {
        unsafe {
            Scm::new(ffi::scm_c_lookup(s.to_c_str().as_ptr()))
        }
    }
    pub fn variable_ref(&self) -> Scm {
        unsafe {
            Scm::new(ffi::scm_variable_ref(self.n))
        }
    }
}

pub fn init() {
    unsafe {
        ffi::scm_init_guile();
    }
}

pub fn c_primitive_load(s: &str) {
    unsafe {
        ffi::scm_c_primitive_load(s.to_c_str().as_ptr());
    }
}
