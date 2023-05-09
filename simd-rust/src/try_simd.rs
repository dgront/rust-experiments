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


#[inline]
#[allow(unsafe_code)]
pub fn dp(a: &[f32],b: &[f32]) {
    assert_eq!(b.len(), a.len());

    if is_x86_feature_detected!("avx") {
        println!("Rolling on AVX!")

    } else {
        println!("AVX not available")
    }
}

pub fn main() {
    let x: Vec<f32> = vec![0.0;8];
    let y: Vec<f32> = vec![0.0;8];

    dp(&x, &y);
}

