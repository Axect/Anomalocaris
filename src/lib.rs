use std::os::raw::{c_double, c_ulong};
use std::ops::Mul;

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
    fn matmul_(m: *const ARMAT, n: *const ARMAT) -> ARMAT;
    fn col_(m: *const ARMAT, k: u64) -> ARMAT;
    fn row_(m: *const ARMAT, k: u64) -> ARMAT;
}

pub fn zeros(m: usize, n: usize) -> ARMAT {
    unsafe {
        zeros_(m as u64, n as u64)
    }
}

impl ARMAT {
    pub fn det(&self) -> f64 {
        unsafe {
            det_(self)
        }
    }
}

impl Mul<ARMAT> for ARMAT {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unsafe {
            matmul_(&self, &rhs)
        }
    }
}
