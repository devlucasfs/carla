use std::ffi::{CString, c_char};

unsafe extern "C" {
    fn c_runa_init(filename: *mut c_char);
    fn c_runa_deinit();
    fn c_runa_constexpr(expr: *mut c_char) -> bool;
    fn c_runa_starts(expr: *mut c_char);
}

pub struct Runa;
impl Runa {
    pub fn init<T: ToString>(filename: T) {
        unsafe {
            let c = CString::new(filename.to_string().as_str()).unwrap();
            let ptr: *mut c_char = c.as_ptr() as *mut c_char;
            c_runa_init(ptr);
        }
    }

    pub fn deinit() {
        unsafe { c_runa_deinit(); }
    }

    pub fn starts<T: ToString>(expr: T) {
        unsafe {
            let c = CString::new(expr.to_string().as_str()).unwrap();
            let ptr: *mut c_char = c.as_ptr() as *mut c_char;
            c_runa_starts(ptr);
        }
    }

    pub fn constexpr<T: ToString>(expr: T) -> bool {
        unsafe {
            let c = CString::new(expr.to_string().as_str()).unwrap();
            let ptr: *mut c_char = c.as_ptr() as *mut c_char;
            c_runa_constexpr(ptr)
        }
    }
}
