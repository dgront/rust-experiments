//! Example rpgram that uses SIMD instruction
//!
//! To compile on M1 Mac:
//! rustup target install x86_64-apple-darwin
//! cargo build --release --target x86_64-apple-darwin
//!
// Documentation:
// https://doc.rust-lang.org/core/arch/x86/
// SIMD and Rust:
// https://state.smerity.com/direct/smerity/state/01E9C39Q6T7XSHFA5C23EMSQRW
// Example Matrix4x4 on SIMD:
// https://gist.github.com/rygorous/4172889

use rand::Rng;
use core::arch::x86_64::*;

#[inline]
#[allow(unsafe_code)]
pub fn check_device() -> bool {

    if is_x86_feature_detected!("avx") {
        println!("Rolling on AVX!");
	return true;

    } else {
        println!("AVX not available");
	return false;
    }
}



#[allow(unsafe_code)]
#[inline(always)]
pub unsafe fn dp(a: &[f32],b: &[f32]) -> f32 {

    assert_eq!(b.len(), a.len());

    let ptr_a = a.as_ptr();
    let ptr_b = b.as_ptr();
    let ptr_a_i = a.as_ptr() as *mut i8;
    let ptr_b_i = b.as_ptr() as *mut i8;

    let n_iter = (a.len() / 8) as isize;
    let mut sum = _mm256_setzero_ps();
    for i in 0..n_iter {

        if i < (n_iter - 1) {
            _mm_prefetch(ptr_a_i.offset(8 * (i + 1)), _MM_HINT_T0);
            _mm_prefetch(ptr_b_i.offset(8 * (i + 1)), _MM_HINT_T0);
        }

	println!("{}",i);
	let mut a_vec: __m256 = _mm256_loadu_ps(ptr_b.offset(8 * i) as *mut f32);
	println!("{} {:?}",i, a_vec);
	let b_vec: __m256 = _mm256_loadu_ps(ptr_a.offset(8 * i) as *mut f32);
	println!("{} {:?}",i, b_vec);
	a_vec = _mm256_sub_ps(a_vec, b_vec);
	println!("{} {:?}",i, a_vec);
	a_vec = _mm256_mul_ps(a_vec, a_vec);
	sum = _mm256_add_ps(a_vec, sum);
	println!("sum: {} {:?}",i, sum);
    }

    let cvlo = _mm256_extractf128_ps(sum, 0);
    let cvhi = _mm256_extractf128_ps(sum, 1);
    let x = _mm_extract_ps::<2>(cvhi);
    let result = f32::from_bits(x as u32);

    return result;
}

pub fn main() {
    const N: usize = 32;
    let mut x: Vec<f32> = vec![0.0;N];
    let mut y: Vec<f32> = vec![0.0;N];
    
    let mut rng = rand::thread_rng();
    for i in 0..N {
	x[i] = rng.gen_range(-1.0..1.0);
	y[i] = rng.gen_range(-1.0..1.0);
    }
    
    if !check_device() { return; }
    else {
	unsafe {
    	    dp(&y, &x);
	}
    }
}

