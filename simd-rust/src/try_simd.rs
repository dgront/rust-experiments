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

#[inline(always)]
unsafe fn _mm256_reduce_add_ps(x: __m256) -> f32 {
    // this is fine since AVX is a superset of SSE - meaning we are guaranted
    // to have the SSE instructions available to us
    let x128: __m128 = _mm_add_ps(_mm256_extractf128_ps(x, 1), _mm256_castps256_ps128(x));
    let x64: __m128 = _mm_add_ps(x128, _mm_movehl_ps(x128, x128));
    let x32: __m128 = _mm_add_ss(x64, _mm_shuffle_ps(x64, x64, 0x55));
    _mm_cvtss_f32(x32)
}

#[allow(unsafe_code)]
#[inline(always)]
pub unsafe fn dp(a: &[f32],b: &[f32]) -> f32 {

    assert_eq!(b.len(), a.len());

    let ptr_a = a.as_ptr();
    let ptr_b = b.as_ptr();

    let n_iter = (a.len() / 8) as isize;
    let mut sum = _mm256_setzero_ps();
    for i in 0..n_iter {
	println!("{}",i);
	let a_vec: __m256 = _mm256_load_ps(ptr_b.offset(8 * i) as *mut f32);
	println!("{} {:?}",i, a_vec);
	let b_vec: __m256 = _mm256_load_ps(ptr_a.offset(8 * i) as *mut f32);
	println!("{} {:?}",i, b_vec);
	let tmp_vec: __m256 = _mm256_sub_ps(a_vec, b_vec);
	sum = _mm256_fmadd_ps(tmp_vec, tmp_vec, sum);
    }

    let result = self::_mm256_reduce_add_ps(sum);
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

