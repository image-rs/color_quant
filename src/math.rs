#[inline]
pub(crate) fn clamp(a: i32) -> i32 {
    if a < 0 {
        0
    } else if a > 255 {
        255
    } else {
        a
    }
}

#[inline]
fn fract(a: f64) -> f64 {
    if a == 0.0 {
        0.
    } else {
        a % 1.
    }
}

#[inline]
pub(crate) fn round(a: f64) -> f64 {
    let one = 1.0;
    let h = 0.5;
    let f = fract(a);
    if f.is_nan() || f == 0.0 {
        a
    } else if a > 0. {
        if f < h {
            a - f
        } else {
            a - f + one
        }
    } else if -f < h {
        a - f
    } else {
        a - f - one
    }
}

#[inline]
pub(crate) fn abs(a: f64) -> f64 {
    if a.is_sign_positive() {
        return a;
    }

    if a.is_sign_negative() {
        return -a;
    }

    core::f64::NAN
}
