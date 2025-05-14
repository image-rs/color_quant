//! These implementations are based on `num-traits`' [`FloatCore`].
//! They have been adapted to the particular needs of `color_quant` and refined
//! through [feedback].
//!
//! [`FloatCore`]: https://docs.rs/num-traits/0.2.19/num_traits/float/trait.FloatCore.html
//! [feedback]: https://github.com/image-rs/color_quant/pull/24#discussion_r2083587462

#[inline]
pub(crate) fn abs(a: f64) -> f64 {
    if a.is_sign_positive() {
        a
    } else if a.is_sign_negative() {
        -a
    } else {
        core::f64::NAN
    }
}

#[inline]
pub(crate) fn clamp_round(a: f64) -> i32 {
    if a < 0. {
        0
    } else if a > 255. {
        255
    } else {
        (a + 0.5) as i32
    }
}
