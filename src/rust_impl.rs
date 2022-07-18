use std::cmp::min;
use std::slice;

pub fn widening_mul(w: &mut [u64], x: &[u64], y: &[u64], limit: usize) {
    let digits_count = x.len();
    for i in 0..w.len() {
        w[i] = 0;
    }
    for i in 0..digits_count {
        let mut c = 0u64;
        let inner_count = min(digits_count, limit - i);
        for j in 0..inner_count {
            let uv = (x[j] as u128) * (y[i] as u128) + w[i + j] as u128 + c as u128;
            w[i + j] = uv as u64;
            c = (uv >> 64) as u64;
        }
        w[i + inner_count] = c;
    }
}

pub fn widening_mul_256(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    let w = unsafe {
        let ptr = w.as_ptr() as *mut u64;
        slice::from_raw_parts_mut(ptr, w.len() / 8)
    };
    let x = unsafe {
        let ptr = x.as_ptr() as *mut u64;
        slice::from_raw_parts(ptr, x.len() / 8)
    };
    let y = unsafe {
        let ptr = y.as_ptr() as *mut u64;
        slice::from_raw_parts(ptr, y.len() / 8)
    };

    for i in 0..batch {
        let w1 = &mut w[8 * i..8 * i + 8];
        let x1 = &x[4 * i..4 * i + 4];
        let y1 = &y[4 * i..4 * i + 4];
        widening_mul(w1, x1, y1, 8);
    }
}
