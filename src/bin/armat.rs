extern crate anomalocaris;
use anomalocaris::*;
use std::ptr;

fn main() {
    let v = vec![1f64, 2f64, 3f64, 4f64];
    let a = ARMAT {
        data: v.as_ptr(),
        nrow: 2,
        ncol: 2,
    };
    let b = ARMAT {
        data: v.as_ptr(),
        nrow: 2,
        ncol: 2,
    };
    let c = a * b;

    let mut d = Vec::<f64>::with_capacity(4);
    unsafe {
        d.set_len(4);
        ptr::copy(c.data, d.as_mut_ptr(), 4);
    }
    println!("{:?}", d);

    let mut e = Vec::<f64>::with_capacity(2);
    unsafe {
        e.set_len(2);
        ptr::copy(c.col(0).data, e.as_mut_ptr(), 2);
    }
    println!("{:?}", e);

    let mut f = Vec::<f64>::with_capacity(2);
    unsafe {
        f.set_len(2);
        ptr::copy(c.row(1).data, f.as_mut_ptr(), 2);
    }
    println!("{:?}", f);
}
