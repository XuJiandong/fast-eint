use std::mem::transmute;

fn widening_mul(w: &mut [u64], x: &[u64], y: &[u64]) {
    let digits_count = x.len();
    for i in 0..w.len() {
        w[i] = 0;
    }
    for i in 0..digits_count {
        let mut c = 0u64;
        for j in 0..digits_count {
            let uv = (x[j] as u128) * (y[i] as u128) + w[i + j] as u128 + c as u128;
            w[i + j] = uv as u64;
            c = (uv >> 64) as u64;
        }
        w[i + digits_count] = c;
    }
}

pub fn widening_mul_256(w: &mut [u8], x: &[u8], y: &[u8], batch: usize) {
    let w = unsafe { transmute::<&mut [u8], &mut [u64]>(w) };
    let x = unsafe { transmute::<&[u8], &[u64]>(x) };
    let y = unsafe { transmute::<&[u8], &[u64]>(y) };

    for i in 0..batch {
        let w1 = &mut w[8 * i..8 * i + 8];
        let x1 = &x[4 * i..4 * i + 4];
        let y1 = &y[4 * i..4 * i + 4];
        widening_mul(w1, x1, y1);
    }
}
