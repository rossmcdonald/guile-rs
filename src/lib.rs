#![crate_name = "guile"]
#![crate_type = "lib"]
#![feature(globs)]

extern crate libc;
extern crate num;

//use ffi::*;
use ffi::Scm;
use std::mem;

use num::Bounded;
use std::ffi::CString;
use std::ptr;
use std::ops::Fn;

mod ffi;

pub static Scm_T: Scm = Scm {
    n: ffi::N {
        n: 1028
    }
};

pub static Scm_F: Scm = Scm {
    n: ffi::N {
        n: 4
    }
};

pub static Scm_UNSPECIFIED: Scm = Scm {
    n: ffi::N {
        n: 2052
    }
};

impl PartialEq for Scm {
    fn eq(&self, other: &Scm) -> bool {
        self.equal(*other)
    }
}

impl PartialOrd for Scm {
    fn partial_cmp(&self, other: &Scm) -> Option<Ordering> {
        if *self < *other  {
            Some(Less)
        } else if *self > *other {
            Some(Greater)
        } else if *self == *other {
            Some(Equal)
        } else {
            None
        }
    }
    fn lt(&self, other: &Scm) -> bool {
        self.less(*other)
    }
    fn le(&self, other: &Scm) -> bool {
        self.less_or_equal(*other)
    }
    fn gt(&self, other: &Scm) -> bool {
        self.greater(*other)
    }
    fn ge(&self, other: &Scm) -> bool {
        self.greater_or_equal(*other)
    }
}

impl ToPrimitive for Scm {
    fn to_i64(&self) -> Option<i64> {
        if self.in_range::<i64>() {
            unsafe {
                Some(ffi::scm_to_int64(self.n))
            }
        } else {
            None
        }
    }
    fn to_u64(&self) -> Option<u64> {
        if self.in_range::<u64>() {
            unsafe {
                Some(ffi::scm_to_uint64(self.n))
            }
        } else {
            None
        }
    }
    fn to_int(&self) -> Option<int> {
        if self.in_range::<int>() {
            unsafe {
                Some(ffi::scm_to_int32(self.n) as int)
            }
        } else {
            None
        }
    }
    fn to_i8(&self) -> Option<i8> {
        if self.in_range::<i8>() {
            unsafe {
                Some(ffi::scm_to_int8(self.n))
            }
        } else {
            None
        }
    }

    fn to_i16(&self) -> Option<i16> {
        if self.in_range::<i16>() {
            unsafe {
                Some(ffi::scm_to_int16(self.n))
            }
        } else {
            None
        }
    }
    fn to_i32(&self) -> Option<i32> {
        if self.in_range::<i32>() {
            unsafe {
                Some(ffi::scm_to_int32(self.n))
            }
        } else {
            None
        }
    }
    fn to_uint(&self) -> Option<uint> {
        if self.in_range::<uint>() {
            unsafe {
                Some(ffi::scm_to_uint32(self.n) as uint)
            }
        } else {
            None
        }
    }
    fn to_u8(&self) -> Option<u8> {
        if self.in_range::<u8>() {
            unsafe {
                Some(ffi::scm_to_uint8(self.n))
            }
        } else {
            None
        }
    }
    fn to_u16(&self) -> Option<u16> {
        if self.in_range::<u16>() {
            unsafe {
                Some(ffi::scm_to_uint16(self.n))
            }
        } else {
            None
        }
    }
    fn to_u32(&self) -> Option<u32> {
        if self.in_range::<u32>() {
            unsafe {
                Some(ffi::scm_to_uint32(self.n))
            }
        } else {
            None
        }
    }
    fn to_f32(&self) -> Option<f32> {
        if self.in_range::<f32>() {
            unsafe {
                Some(ffi::scm_to_double(self.n) as f32)
            }
        } else {
            None
        }
    }
    fn to_f64(&self) -> Option<f64> {
        if self.in_range::<f64>() {
            unsafe {
                Some(ffi::scm_to_double(self.n) as f64)
            }
        } else {
            None
        }
    }
}

pub trait ToScm {
    fn to_scm(&self) -> Scm;
}

impl ToScm for uint {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_uint32(*self as u32)
        }
    }
}

impl ToScm for int {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_int32(*self as i32)
        }
    }
}

impl ToScm for i8 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_int8(*self)
        }
    }
}

impl ToScm for u8 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_uint8(*self)
        }
    }
}


impl ToScm for i16 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_int16(*self)
        }
    }
}

impl ToScm for u16 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_uint16(*self)
        }
    }
}

impl ToScm for i32 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_int32(*self)
        }
    }
}

impl ToScm for u32 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_uint32(*self)
        }
    }
}


impl ToScm for i64 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_int64(*self)
        }
    }
}

impl ToScm for u64 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_uint64(*self)
        }
    }
}

impl ToScm for f32 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_double(*self as f64)
        }
    }
}

impl ToScm for f64 {
    fn to_scm(&self) -> Scm {
        unsafe {
            ffi::scm_from_double(*self)
        }
    }
}

impl Scm {
    fn in_range<T: Bounded+ToScm>(&self) -> bool {
        let min: T = Bounded::min_value();
        let max: T = Bounded::max_value();
        min.to_scm() <= *self && *self <= max.to_scm()
    }

    pub fn is_true(&self) -> bool {
        let a: uint = 260;
        let b: uint = 4;
        !((self.n.n & !(a ^ b)) == (a & b))
    }

    pub fn call(&self, args: Vec<Box<ToScm>>) -> Scm {
        let mut s: Vec<Scm> = args.iter().map(|x| x.to_scm()).collect();
        unsafe {
            let len = s.len();
            ffi::scm_call_n(
                self.n, s.as_mut_slice(), len as u64)
        }
    }
    pub fn c_lookup(s: &str) -> Scm {
        unsafe {
            ffi::scm_c_lookup(s.to_c_str().as_ptr())
        }
    }
    pub fn variable_ref(&self) -> Scm {
        unsafe {
            ffi::scm_variable_ref(self)
        }
    }

    pub fn add(&self, x: Scm) -> Scm {
        unsafe {
            ffi::scm_sum(self, x)
        }
    }

    // equality

    fn equal(&self, x: Scm) -> bool {
        unsafe {
            ffi::scm_num_eq_p(self, x).is_true()
        }
    }
    fn less(&self, x: Scm) -> bool {
        unsafe {
            ffi::scm_less_p(self, x).is_true()
        }
    }
    fn greater(&self, x: Scm) -> bool {
        unsafe {
            ffi::scm_gr_p(self, x).is_true()
        }
    }
    fn less_or_equal(&self, x: Scm) -> bool {
        unsafe {
            ffi::scm_leq_p(self, x).is_true()
        }
    }
    fn greater_or_equal(&self, x: Scm) -> bool {
        unsafe {
            ffi::scm_geq_p(self, x).is_true()
        }
    }
}

pub fn shell(args: Vec<String>) {
    let c_strings: Vec<CString> = args.iter().map(|s| s.to_c_str()).collect();
    let ptrs = c_strings.iter().map(|cs| cs.as_ptr())
        .chain(Some(ptr::null::<libc::c_char>()).move_iter())
        .collect::<Vec<*const libc::c_char>>();
    unsafe {
        ffi::scm_shell(args.len() as u32, ptrs.as_ptr());
    }
}

pub fn define_gsubr(name: String, req: int, opt: int,
                    rst: int, fcn: fn(i: Scm) -> Box<ToScm>) -> Scm {
    unsafe {
        ffi::scm_c_define_gsubr(name.to_c_str().as_ptr(),
                                       req, opt, rst, fcn)
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



#[cfg(test)]
mod tests {
    use super::*;
    use super::ffi::*;

    #[test]
    fn testgsubr() {
        fn fun() -> Scm {
            1i32.to_scm()
        }
        let s = define_gsubr("bla".to_string(), 0, 0, 0, fun);
        let s = 1i32.to_scm();
        println!("{:?}", s);
        assert!(s.to_i32().unwrap() == 1i32);
    }

    #[test]
    fn equal() {
        let a = 1i32.to_scm();
        let b = 1i32.to_scm();
        let c = 2i32.to_scm();
        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn inequal() {
        let a = 1i32.to_scm();
        let b = 2i32.to_scm();
        assert!(b > a);
        assert!(a < b);
        assert!(a != b);
    }

    #[test]
    fn true_truth() {
        assert!(Scm_T.is_true());
    }

    #[test]
    fn false_falseness() {
        assert!(!Scm_F.is_true());
    }
}
