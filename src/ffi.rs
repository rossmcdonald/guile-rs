use libc::*;

//enum __scm_opaque {} pub type Scm = *const __scm_opaque;
pub struct N {
    pub n: uintptr_t,
}

impl PartialEq for N {
    fn eq(&self, other: &N) -> bool {
        self.n == other.n
    }
}

pub struct Scm {
    pub n: N,
}

/*impl PartialEq for Scm {
    fn eq(&self, other: &Scm) -> bool {
        self.n == other.n
    }
}*/


#[link(name="guile-2.0")]
extern {
    // setup functions
    pub fn scm_init_guile() -> ();
    pub fn scm_shell(argc: c_uint, argv: *const *const c_char) -> ();
    pub fn scm_c_primitive_load(str: *const c_char) -> ();
    pub fn scm_variable_ref(var: Scm) -> Scm;
    pub fn scm_c_lookup(str: *const c_char) -> Scm;

    // calling function
    pub fn scm_call_n(func: Scm, args: &mut [Scm], nargs: size_t) -> Scm;
    //pub fn scm_call(func: Scm, ...) -> Scm;

    // number to object conversion
    pub fn scm_from_int8(x: int8_t) -> Scm;
    pub fn scm_from_uint8(x: uint8_t) -> Scm;
    pub fn scm_from_int16(x: int16_t) -> Scm;
    pub fn scm_from_uint16(x: uint16_t) -> Scm;
    pub fn scm_from_int32(x: int32_t) -> Scm;
    pub fn scm_from_uint32(x: uint32_t) -> Scm;
    pub fn scm_from_int64(x: int64_t) -> Scm;
    pub fn scm_from_uint64(x: uint64_t) -> Scm;
    pub fn scm_from_double(x: c_double) -> Scm;

    // object to number conversion
    pub fn scm_to_int8(x: Scm) -> i8;
    pub fn scm_to_uint8(x: Scm) -> u8;
    pub fn scm_to_int16(x: Scm) -> i16;
    pub fn scm_to_uint16(x: Scm) -> u16;
    pub fn scm_to_int32(x: Scm) -> i32;
    pub fn scm_to_uint32(x: Scm) -> u32;
    pub fn scm_to_int64(x: Scm) -> i64;
    pub fn scm_to_uint64(x: Scm) -> u64;
    pub fn scm_to_double(x: Scm) -> f32;
    pub fn scm_to_bool(x: Scm) -> c_int;

    // comparison functions
    //pub fn scm_i_num_less_p(x: Scm, y: Scm, z: Scm) -> Scm;
    pub fn scm_num_eq_p(x: Scm, y: Scm) -> Scm;
    pub fn scm_less_p(x: Scm, y: Scm) -> Scm;
    pub fn scm_gr_p(x: Scm, y: Scm) -> Scm;
    pub fn scm_leq_p(x: Scm, y: Scm) -> Scm;
    pub fn scm_geq_p(x: Scm, y: Scm) -> Scm;
    pub fn scm_zero_p(z: Scm) -> Scm;
    pub fn scm_positive_p(x: Scm) -> Scm;
    pub fn scm_negative_p(x: Scm) -> Scm;

    // arithmetic operations
    pub fn scm_sum(x: Scm, y: Scm) -> Scm;

    // primitive procedures
    pub fn scm_c_define_gsubr(name: *const c_char, req: int,
                            opt: int, rst: int, fcn: fn(i: Scm) -> Scm) -> Scm;
}
