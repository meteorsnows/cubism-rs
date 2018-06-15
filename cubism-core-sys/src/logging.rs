use libc::c_char;

pub type csmLogFunction = Option<unsafe extern "C" fn(message: *const c_char)>;

extern "C" {
    pub fn csmGetLogFunction() -> csmLogFunction;
    pub fn csmSetLogFunction(handler: csmLogFunction);
}
