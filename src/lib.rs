use std::os::raw::{c_double, c_ulong};

#[derive(Debug)]
#[repr(C)]
pub struct ARMAT {
    pub data: *const c_double,
    pub nrow: c_ulong,
    pub ncol: c_ulong,
}

#[link(name = "arma")]
extern "C" {
    fn zeros_(m: u64, n: u64) -> ARMAT;
    fn det_(m: *const ARMAT) -> f64;
}

pub fn zeros(m: usize, n: usize) -> ARMAT {
    unsafe {
        zeros_(m as u64, n as u64)
    }
}

impl ARMAT {
    fn det(&self) -> f64 {
        unsafe {
            det_(self)
        }
    }
}

