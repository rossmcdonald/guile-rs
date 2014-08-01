use libc::*;

//enum __scm_opaque {} pub type SCM = *const __scm_opaque;
pub struct N {
    pub n: uintptr_t,
}

impl PartialEq for N {
    fn eq(&self, other: &N) -> bool {
        self.n == other.n
    }
}

pub struct SCM {
    pub n: N,
}

impl PartialEq for SCM {
    fn eq(&self, other: &SCM) -> bool {
        self.n == other.n
    }
}


#[link(name="guile-2.0")]
extern {
    // setup functions
    pub fn scm_init_guile() -> ();
    pub fn scm_c_primitive_load(str: *const c_char) -> ();
    pub fn scm_variable_ref(var: SCM) -> SCM;
    pub fn scm_c_lookup(str: *const c_char) -> SCM;

    // calling function
    pub fn scm_call_n(func: SCM, args: &mut [SCM], nargs: size_t) -> SCM;
    //pub fn scm_call(func: SCM, ...) -> SCM;

    // number to object conversion
    pub fn scm_from_int8(x: int8_t) -> SCM;
    pub fn scm_from_uint8(x: uint8_t) -> SCM;
    pub fn scm_from_int16(x: int16_t) -> SCM;
    pub fn scm_from_uint16(x: uint16_t) -> SCM;
    pub fn scm_from_int32(x: int32_t) -> SCM;
    pub fn scm_from_uint32(x: uint32_t) -> SCM;
    pub fn scm_from_int64(x: int64_t) -> SCM;
    pub fn scm_from_uint64(x: uint64_t) -> SCM;
    pub fn scm_from_double(x: c_double) -> SCM;

    // object to number conversion
    pub fn scm_to_int8(x: SCM) -> i8;
    pub fn scm_to_uint8(x: SCM) -> u8;
    pub fn scm_to_int16(x: SCM) -> i16;
    pub fn scm_to_uint16(x: SCM) -> u16;
    pub fn scm_to_int32(x: SCM) -> i32;
    pub fn scm_to_uint32(x: SCM) -> u32;
    pub fn scm_to_int64(x: SCM) -> i64;
    pub fn scm_to_uint64(x: SCM) -> u64;
    pub fn scm_to_double(x: SCM) -> f32;
    pub fn scm_to_bool(x: SCM) -> c_int;

    // comparison functions
    //pub fn scm_i_num_less_p(x: SCM, y: SCM, z: SCM) -> SCM;
    pub fn scm_num_eq_p(x: SCM, y: SCM) -> SCM;
    pub fn scm_less_p(x: SCM, y: SCM) -> SCM;
    pub fn scm_gr_p(x: SCM, y: SCM) -> SCM;
    pub fn scm_leq_p(x: SCM, y: SCM) -> SCM;
    pub fn scm_geq_p(x: SCM, y: SCM) -> SCM;
    pub fn scm_zero_p(z: SCM) -> SCM;
    pub fn scm_positive_p(x: SCM) -> SCM;
    pub fn scm_negative_p(x: SCM) -> SCM;
    //pub fn scm_is_true(x: SCM) -> bool;

}
