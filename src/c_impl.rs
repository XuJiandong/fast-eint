#[link(name = "fast-eint-c-impl", kind = "static")]
extern "C" {
    pub fn fe_widening_mul_256_batch(w: *mut u64, x: *const u64, y: *const u64, batch: isize);
}

pub fn widening_mul_256(w: &mut [u8], x: &[u8], y: &[u8], batch: isize) {
    unsafe {
        fe_widening_mul_256_batch(
            w.as_ptr() as *mut u64,
            x.as_ptr() as *const u64,
            y.as_ptr() as *const u64,
            batch,
        );
    }
}
