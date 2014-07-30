use libc::*;

pub enum __scm_opaque {} type SCM = *const __scm_opaque;

#[link(name="guile-2.0")]
extern {
    pub fn scm_init_guile() -> ();
    pub fn scm_c_primitive_load(str: *const c_char) -> ();
    pub fn scm_variable_ref(var: SCM) -> SCM;
    pub fn scm_c_lookup(str: *const c_char) -> SCM;
    pub fn scm_call_1(func: SCM, var: SCM) -> SCM;
    pub fn scm_from_int32(num: c_int) -> SCM;
    pub fn scm_to_int32(scm: SCM) -> c_int;
}
