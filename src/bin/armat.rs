extern crate anomalocaris;
use anomalocaris::*;
use std::ptr;

fn main() {
    let v = vec![1f64; 4];
    let a = ARMAT {
        data: v.as_ptr(),
        nrow: 2,
        ncol: 2,
    };
    let b = unsafe {
        zeros(2, 2)
    };

    let mut c = Vec::<f64>::with_capacity(4);
    unsafe {
        c.set_len(4);
        ptr::copy(a.data, c.as_mut_ptr(), 4);
    }
    println!("{:?}", c);
}
